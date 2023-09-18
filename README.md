# nix-cache-check

[![Integration](https://github.com/holochain/nix-cache-check/actions/workflows/integration.yml/badge.svg)](https://github.com/holochain/nix-cache-check/actions/workflows/integration.yml)

A GitHub action to check whether a Nix flake is properly cached. It builds
a derivation and fails if anything needs building rather than fetching
from a remote cache. You can permit some derivations to be built using the
`permit_build_derivations` option.

_Health warning_: This action is scanning the output of `nix build`.

### Example usage

```
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: cachix/install-nix-action@v22
      - uses: cachix/cachix-action@v12
        with:
          name: my-cache
      - name: Check the cache
        uses: holochain/nix-cache-check@v1
        with:
          derivation: .#my-derivation
```

There is also a working example (as long as the build is currently passing!) in the [integration test](https://github.com/holochain/nix-cache-check/blob/main/.github/workflows/integration.yml) for this action.
