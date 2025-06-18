[group("publish")]
publish_all:
  just publish "maybe-fut-unwrap-derive"
  just publish "maybe-fut-io-derive"
  just publish "maybe-fut-derive"
  just publish "maybe-fut"

[group("publish")]
publish crate:
  #!/usr/bin/env bash

  echo "Publishing {{crate}}"
  if just crate_outdated {{crate}}; then
    set -ex
    cargo publish --dry-run --allow-dirty --package {{crate}}
  else
    echo "Crate {{crate}} is already up to date. Skipping publish."
  fi

[private]
crate_outdated crate:
  #!/usr/bin/env bash
  
  crate_version=$(cargo metadata --no-deps --format-version 1 | jq -r ".packages[] | select(.name == \"{{crate}}\").version")
  published_versions=$(curl -s "https://crates.io/api/v1/crates/{{crate}}" | jq -r '.versions[].num')
  if [ $? -ne 0 ]; then
    echo "Crate {{crate}} has never been published. Publishing now."
    exit 0
  fi

  if echo "$published_versions" | grep -q "^$crate_version$"; then
    echo "Crate {{crate}} is up to date."
    exit 1
  else
    echo "Crate {{crate}} is not up to date. Current version: $crate_version"
    exit 0
  fi
