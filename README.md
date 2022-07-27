w# anchor-hello-world
anchor-hello-world



# Get Started

## Key Path
```
Windows WSL: /home/iml1111/.config/solana/id.json
```


## Deploy

check the `Anchor.toml`.
```
$ anchor build

# 동적으로 생성된 키 페어의 퍼블릭키 가져오기
$ anchor keys list

# declare_id!() 및 Anchor.toml의 program_id를 갱신.

# 컨트랙트 배포하기
$ anchor deploy

# Second Shell
$ solana-test-validator
```



# Deployments(devnet)



## hello-world

- DBx3vVhSEtFpqhUxe6HcHLiEbCCk32nsKzH8ErGTjxN9

### IDL
```json
{
  "version": "0.1.0",
  "name": "hello_world",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [],
      "args": []
    }
  ],
  "metadata": {
    "address": "DBx3vVhSEtFpqhUxe6HcHLiEbCCk32nsKzH8ErGTjxN9"
  }
}
```

## incrementers
- 9UTBXFVBZcApvmJmxWtwKUHZZ6eEQc4mXRWv5NQ1DuYg

### IDL
```json
{
  "version": "0.1.0",
  "name": "incrementer",
  "instructions": [
    {
      "name": "create",
      "accounts": [
        {
          "name": "baseAccount",
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
      "args": []
    },
    {
      "name": "increment",
      "accounts": [
        {
          "name": "baseAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "BaseAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    }
  ]
}

```

## simple-nft
- 4inCVpyXV31tv7Z9XXiEVoNBumWX5VYacmgmbDa9xsMk

### INFO
```
# Token 1 (iml1 혼자서 sign)
User: 6wAqtFA7Z8ALxSq4j3DMh1wUj6vmCMmoCAKZzXP18pmk
Token Acc: 7sA23bjojeT1kDo43wpXp6g3yHT51iXvvzUC9pCM7PEZ
Token: 9iHzDX4xn7Nidw9HF7F5tjCp61EEBLAbsh27fSjPRNQX
Metdata Acc: EQAJNKM9PZw22fMCcqvo5e3RNbtp81PcPq6srSCALy16
Master Edition Acc: F87owwas8hL1BipgHy7Cx1Es23CiHhwxiDUfJ7WdZDVd
49tMERQ4skrvgvLvjCdj9aHeF4UzAqtX25fc2TYAK2bNpaTYaGKtVNDYiyXurkg5K7BzRZ6FpQPoBmKpGiVzexy5

# Token 2 (iml2 혼자서 sign)
User: CqeESEVSgUbjjTw1TLbBq8ZWq5AZy6aE62FPAS4UZC6d
Token Acc: 7VQwnhgsYsTZCxTv6XmmEqGAPQvKaYnSgzh6GNeNR1MT
Token: CmvjaWC48XktsSwsnZCKWE2U1BqASKWb2ow4SYXFiRGQ
Metdata Acc: CqhsYeg1ZR1zgCWap3z7KUSLjhhjEQVdxFWhpjUhUZeD
Master Edition Acc: 6F2xTSaPAnf5JnEkidorLGUXH1JULzDMssTd3rHjHBEz
SrbSdQNdoNSCdN6cop6hj6kp7Joy86e8ye1QNtH3bniGb3E6A1rjGYM3aoGZpFHrgCEXUK8PZw3uDgr8b3TaeyS

# Token 3 (iml2가 발급하지만 iml1이 seller fee를 탈취함)
User: CqeESEVSgUbjjTw1TLbBq8ZWq5AZy6aE62FPAS4UZC6d
Token Acc: ZdvZJehxbYT3YDmcXDfZMPNtmTQc2U5ETxosetS4hYs
Token: 6mjne4fXZthWkNpxvjpZp69FauM4wSaY4QAaVQi7K4wP
Metdata Acc: BB2QwjmLg7NaHUM9UacNfsrki7P8jfCscUhhsBpbPPig
Master Edition Acc: DwVLWtgsfZrW7uxSZv731SHSxZzwBGiJFfyJMcWPctYW
2BEytt9iBkButVPW7c3tcZnDj7worWLTTveT8khaHZ4ecANgG19CryTZjLjY94U5mRkEQZT3N8FhPPioLrJpBH9Q
```

### IDL
```
{
  "version": "0.1.0",
  "name": "simple_nft",
  "instructions": [
    {
      "name": "mintNft",
      "accounts": [
        {
          "name": "mintAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        }
      ]
    }
  ]
}
```