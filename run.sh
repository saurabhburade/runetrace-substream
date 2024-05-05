export SUBSTREAMS_API_TOKEN="eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjM2MDAwMDAwMTcxMzY5NjcwOCwianRpIjoiZWY2Mzg0ZmItYjY3ZC00MGE5LTg5NGYtZDllYzEzMzRhNzVkIiwiaWF0IjoxNzEzNjk2NzA4LCJpc3MiOiJkZnVzZS5pbyIsInN1YiI6IjBzdWhhZTU1Y2MwNjJiZDI3YjNmZCIsInYiOjEsImFraSI6ImU0NTZkOTdiMzNkOWQyNTllYTVlZGYwNmM2MTE1ZGUyZDVjZDIxNGM4MTNjOTZhODEwMDg1OTBhMWJhZjg1YzAiLCJ1aWQiOiIwc3VoYWU1NWNjMDYyYmQyN2IzZmQifQ.I6VpZ5haf-mpVU7ipSt0CB_X0RA9-XRPvcihXjm_HCihDZcfbXCXJ3vd2_eP6lrkgBUB3PbVd8Zw-4_KJYN_dw"
# 8d997976bd532a01b6cfb767445533b78937172177353fb7 API_KEY_PINAX
# server_7cfdb0684c0a3757084f091e72949700 API_KEY_STREAMINGFAST
# export SUBSTREAMS_API_KEY="8d997976bd532a01b6cfb767445533b78937172177353fb7"
# substreams gui -e bitcoin.substreams.pinax.network:443 ipfs://QmTD6sqr14t5hwWxmFGK4FiptyRY6yBHNsxUGmc1VVHSQP map_ordinals  --stop-block +5 --production-mode
# substreams gui -e bitcoin.substreams.pinax.network:443 ipfs://QmQ9w8sBUobcynfrfQJrBazDJ82xW5JX8HZ9h4i2iFxois map_ordinals  --stop-block +5 --production-mode
export SUBSTREAMS_API_KEY="server_7cfdb0684c0a3757084f091e72949700"
# substreams gui -e mainnet.btc.streamingfast.io:443 ipfs://QmUmUbrYaUxDBUXmi4teUACVi6tpQ48NwnV7eNQKC9Z7GS map_ordinals  --stop-block +5 --production-mode
# substreams gui -e mainnet.btc.streamingfast.io:443 ipfs://QmQ9w8sBUobcynfrfQJrBazDJ82xW5JX8HZ9h4i2iFxois map_ordinals  --stop-block +5 --production-mode
# substreams run -e bitcoin.substreams.pinax.network:443 \
#    substreams.yaml \
#    map_runes \
#    --start-block 840000 \
#    --stop-block +1

substreams gui -e mainnet.btc.streamingfast.io:443 \
   substreams.yaml \
   map_ordinals \
   --start-block 840000 \
   --stop-block +5 \
   --production-mode

# substreams gui -e mainnet.btc.streamingfast.io:443 \
#    substreams.yaml \
#    map_inscriptions \
#    --start-block 780309 \
#    --stop-block +100000

# substreams gui -e mainnet.btc.streamingfast.io:443 \
#    substreams.yaml \
#    map_transactions \
#    --start-block 840000 \
#    --stop-block +1
# substreams gui -e mainnet.btc.streamingfast.io:443 \
#    substreams.yaml \
#    map_runes \
#    --start-block 840000 \
#    --stop-block +5
   # map_runes \
