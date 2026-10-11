# OpenBao, profile S: one node on raft storage behind a TLS listener. Unsealing is a ceremony
# (Shamir 3-of-5, SP-12); `make bao-dev` initialises 1-of-1 for development only. OpenBao no longer locks
# memory itself: the host runs with swap off (or encrypted), as it does for the AKR binaries.
ui            = false
api_addr      = "https://openbao:8200"
cluster_addr  = "https://openbao:8201"

storage "raft" {
  path    = "/openbao/file"
  node_id = "akr-bao-1"
}

listener "tcp" {
  address       = "0.0.0.0:8200"
  tls_cert_file = "/run/akr/tls/openbao.crt"
  tls_key_file  = "/run/akr/tls/openbao.key"
}
