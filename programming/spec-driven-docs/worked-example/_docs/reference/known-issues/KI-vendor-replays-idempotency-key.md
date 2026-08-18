---
upstream: https://github.com/example/vendor-sdk/issues/1234
affects: upload client
state: masked
workaround: treat an empty 200 body as the cached result rather than a failed write
retire_when: vendor-sdk release >= 2.4.0
---

# The vendor returns 200 with an empty body on a replayed idempotency key

## Symptom

A retried upload receives HTTP 200 with a zero-length body. The client cannot tell the replay apart
from a successful write that returned nothing, so a naive parse raises on the empty document.

## Signal

| Signal                                   | Expected result                                  |
| ---------------------------------------- | ------------------------------------------------ |
| `rg 'PUT /artifacts' access.log \| tail` | two entries sharing one `Idempotency-Key` header |
| response `Content-Length`                | `0` on the second entry, non-zero on the first   |

## Workaround

`src/cleanup.py` treats an empty body as the cached result. The comment beside it states the vendor
invariant, because no rule of this project was agreed to produce that branch.

`tests/test_cleanup.py` carries a strict expected failure naming this case, so the suite turns red
the moment the vendor ships the fix and the mask can be removed.
