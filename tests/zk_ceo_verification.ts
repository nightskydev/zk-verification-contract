export type ZkCeoVerification = {
  "version": "0.1.0",
  "name": "zk_ceo_verification",
  "instructions": [
    {
      "name": "initializeCompany",
      "accounts": [
        {
          "name": "company",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "registrationNumber",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "ceoPublicKey",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "verifyCeo",
      "accounts": [
        {
          "name": "company",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": {
            "defined": "CEOProof"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "company",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "registrationNumber",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "ceoPublicKey",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CEOProof",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proofA",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "proofB",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          },
          {
            "name": "proofC",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "publicSignals",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "DeserializationFailed"
    },
    {
      "code": 6001,
      "name": "VerificationFailed"
    },
    {
      "code": 6002,
      "name": "GameAlreadyOver"
    },
    {
      "code": 6003,
      "name": "NotPlayersTurn"
    },
    {
      "code": 6004,
      "name": "OnlyHostOpeningShot"
    },
    {
      "code": 6005,
      "name": "GameAlreadyStarted"
    },
    {
      "code": 6006,
      "name": "BoardZKVerificationFailed"
    },
    {
      "code": 6007,
      "name": "ShotZKVerificationFailed"
    },
    {
      "code": 6008,
      "name": "GameNotJoinable"
    },
    {
      "code": 6009,
      "name": "FirstTurnShouldBePlayed"
    },
    {
      "code": 6010,
      "name": "GameStillOngoing"
    },
    {
      "code": 6011,
      "name": "NotPlayer"
    }
  ]
};

export const IDL: ZkCeoVerification = {
  "version": "0.1.0",
  "name": "zk_ceo_verification",
  "instructions": [
    {
      "name": "initializeCompany",
      "accounts": [
        {
          "name": "company",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "registrationNumber",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "ceoPublicKey",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "verifyCeo",
      "accounts": [
        {
          "name": "company",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": {
            "defined": "CEOProof"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "company",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "registrationNumber",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "ceoPublicKey",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CEOProof",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proofA",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "proofB",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          },
          {
            "name": "proofC",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "publicSignals",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "DeserializationFailed"
    },
    {
      "code": 6001,
      "name": "VerificationFailed"
    },
    {
      "code": 6002,
      "name": "GameAlreadyOver"
    },
    {
      "code": 6003,
      "name": "NotPlayersTurn"
    },
    {
      "code": 6004,
      "name": "OnlyHostOpeningShot"
    },
    {
      "code": 6005,
      "name": "GameAlreadyStarted"
    },
    {
      "code": 6006,
      "name": "BoardZKVerificationFailed"
    },
    {
      "code": 6007,
      "name": "ShotZKVerificationFailed"
    },
    {
      "code": 6008,
      "name": "GameNotJoinable"
    },
    {
      "code": 6009,
      "name": "FirstTurnShouldBePlayed"
    },
    {
      "code": 6010,
      "name": "GameStillOngoing"
    },
    {
      "code": 6011,
      "name": "NotPlayer"
    }
  ]
};
