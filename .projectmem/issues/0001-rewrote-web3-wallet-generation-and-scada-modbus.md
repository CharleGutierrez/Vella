# #0001 Rewrote Web3 wallet generation and SCADA Modbus client to use real cryptographic key derivation (k256, sha3) and genuine async TCP bindings (tokio-modbus) instead of mocked values

- 2026-08-26T21:56:20Z `issue`: Rewrote Web3 wallet generation and SCADA Modbus client to use real cryptographic key derivation (k256, sha3) and genuine async TCP bindings (tokio-modbus) instead of mocked values
- 2026-08-26T21:56:25Z `fix`: Made mocked features real: embedded wallet manager now derives real ETH addresses, SCADA modbus connects to real TCP endpoints
