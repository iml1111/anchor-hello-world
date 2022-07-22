# anchor-hello-world
anchor-hello-world



# Get Started



## Deploy

check the `Anchor.toml`.
```
# 동적으로 생성된 키 페어의 퍼블릭키 가져오기
$ solana address -k ./target/deploy/keypair.json

# declare_id!() 및 Anchor.toml의 program_id를 갱신.

# 컨트랙트 배포하기
$ anchor deploy

# Second Shell
$ solana-test-validator
```

## Test
```
$ anchor test
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