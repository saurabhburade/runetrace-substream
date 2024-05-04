export SUBSTREAMS_API_TOKEN="eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjM2MDAwMDAwMTcxMzY5NjcwOCwianRpIjoiZWY2Mzg0ZmItYjY3ZC00MGE5LTg5NGYtZDllYzEzMzRhNzVkIiwiaWF0IjoxNzEzNjk2NzA4LCJpc3MiOiJkZnVzZS5pbyIsInN1YiI6IjBzdWhhZTU1Y2MwNjJiZDI3YjNmZCIsInYiOjEsImFraSI6ImU0NTZkOTdiMzNkOWQyNTllYTVlZGYwNmM2MTE1ZGUyZDVjZDIxNGM4MTNjOTZhODEwMDg1OTBhMWJhZjg1YzAiLCJ1aWQiOiIwc3VoYWU1NWNjMDYyYmQyN2IzZmQifQ.I6VpZ5haf-mpVU7ipSt0CB_X0RA9-XRPvcihXjm_HCihDZcfbXCXJ3vd2_eP6lrkgBUB3PbVd8Zw-4_KJYN_dw"

# substreams gui -e mainnet.btc.streamingfast.io:443 \
#    substreams.yaml \
#    map_ordinals \
#    --start-block 779468 \
#    --stop-block +100000

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
substreams gui -e mainnet.btc.streamingfast.io:443 \
   substreams.yaml \
   map_runes \
   --start-block 840000 \
   --stop-block +5
   # map_runes \
