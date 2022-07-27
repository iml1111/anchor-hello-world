# anchor-hello-world
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
- 4jvMtVYUSyWoVvaxwAnWzSNdqaZHJArKG3zyFQGZjuty

### INFO
```
# Token 1
mint: Gsqwt9tfYDBix9kq7Uqq4hMREwxb83SjN8Js9H3469rp (IML token)
user_account: CqeESEVSgUbjjTw1TLbBq8ZWq5AZy6aE62FPAS4UZC6d (iml2)
token_account: 8nhf5o82VQo1NSxj6QjUenS72mnGEG7QotrxvzEpeovT (iml2' token account)

# Token 2
User: 6wAqtFA7Z8ALxSq4j3DMh1wUj6vmCMmoCAKZzXP18pmk
Token Acc: 8FSSobV4pjNceNV44s39b3VRnFTdCGgtTcCgSKSGTwjK
Token: E1fnFLsZP6iADPQxeYEKxPLEhGVADGcthuVyQdzLsKhK
Metdata Acc: FmKjMfg5k5A3kbjJtYY6DRERqmxGfSgCNbYEQ3iAPDmM
Master Edition Acc: GqtW8D9VGtyby1nPXuhboGDF7s4FeWQ5rkhD4sbM45tg
3kWZUV71sLWL8nWKVucKM6F5K27aWzhGxiG9TPygiMoBHWS465dSowehpKG3DMdduqtuK2yRMPSUbXPR7LqW1ew7
```

### IDL
```
Not yet.
```