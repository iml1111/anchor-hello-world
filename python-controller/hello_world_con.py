import asyncio
from anchorpy import Provider
from anchorpy import Provider, Wallet
from solana.keypair import Keypair
from solana.transaction import Transaction
from solana.rpc.async_api import AsyncClient
from hello_world.instructions import initialize


def main():
    temp_acc = Keypair.from_secret_key([153,31,111,247,54,72,131,173,86,83,147,99,149,19,193,34,33,168,163,54,121,60,212,35,21,169,200,137,215,254,204,147,175,230,136,112,244,175,75,84,120,46,136,90,208,86,228,223,197,166,39,82,73,9,73,198,159,59,242,77,93,38,17,34])
    print("Pubkey:", temp_acc.public_key)
    ix = initialize()
    tx = Transaction().add(ix)
    
    client = AsyncClient(endpoint='https://api.devnet.solana.com')
    wallet = Wallet(payer=temp_acc)
    
    provider = Provider(
        connection=client,
        wallet=wallet
    )
    res = asyncio.run(provider.send(tx, signers=[temp_acc]))
    print(res)


if __name__ == "__main__":
    main()