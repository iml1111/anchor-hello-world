import asyncio
from anchorpy import Provider, Wallet
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.transaction import Transaction
from solana.rpc.async_api import AsyncClient
from solana.rpc.core import RPCException
from pprint import pprint
from metaplex.metadata import get_metadata_account, get_edition

from simple_nft.instructions import mint_nft, MintNftArgs, MintNftAccounts

from solana.system_program import SYS_PROGRAM_ID
from solana.sysvar import SYSVAR_RENT_PUBKEY
from metaplex.metadata import METADATA_PROGRAM_ID, TOKEN_PROGRAM_ID


IML1 = [56,148,44,55,92,31,202,61,172,155,168,205,29,201,251,85,44,146,153,130,45,53,215,32,123,217,163,232,103,25,243,28,88,42,237,121,92,76,242,132,78,37,138,43,165,144,0,84,230,130,170,237,171,132,206,241,123,23,139,51,51,79,104,151]

IML2 = [153,31,111,247,54,72,131,173,86,83,147,99,149,19,193,34,33,168,163,54,121,60,212,35,21,169,200,137,215,254,204,147,175,230,136,112,244,175,75,84,120,46,136,90,208,86,228,223,197,166,39,82,73,9,73,198,159,59,242,77,93,38,17,34]

URL = "https://raw.githubusercontent.com/iml1111/iml1111.github.io/main/iml_token.png"
# Image Print TODO


def main():

    # 토큰 발행자만 실행 가능함
    user_acc = Keypair.from_secret_key(IML1)
    token_acc = PublicKey("8nhf5o82VQo1NSxj6QjUenS72mnGEG7QotrxvzEpeovT")
    token = PublicKey("Gsqwt9tfYDBix9kq7Uqq4hMREwxb83SjN8Js9H3469rp")
    metadata_acc = get_metadata_account(token)
    master_edition_acc = get_edition(token)
    
    print("User:", user_acc.public_key)
    print("Token Acc:", token_acc)
    print("Token:", token)
    print("Metdata Acc:", metadata_acc)
    print("Master Edition Acc:", master_edition_acc)

    ix = mint_nft(
        MintNftArgs(
            creator_key=token, # TODO Creator remove
            uri=URL,
            title="IML NFT",
            symbol="IMIML",
        ),
        MintNftAccounts(
            mint_authority=user_acc.public_key,
            system_program=SYS_PROGRAM_ID,
            token_program=TOKEN_PROGRAM_ID,
            mint=token,
            metadata=metadata_acc,
            payer=user_acc.public_key,
            token_account=token_acc, 
            master_edition=master_edition_acc,
            token_metadata_program=METADATA_PROGRAM_ID,
            rent=SYSVAR_RENT_PUBKEY,
        )
    )
    tx = Transaction().add(ix)
    client = AsyncClient(endpoint='https://api.devnet.solana.com')
    wallet = Wallet(payer=user_acc)
    provider = Provider(
        connection=client,
        wallet=wallet,
    )
    try:
        res = asyncio.run(provider.send(tx, [iml2]))
    except RPCException as e:
        error = e.args[0]
        pprint(error)

    else:
        print(res)



if __name__ == '__main__':
    main()