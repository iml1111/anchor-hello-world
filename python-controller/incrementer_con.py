
import asyncio
from anchorpy import Provider
from anchorpy import Provider, Wallet
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.transaction import Transaction
from solana.rpc.async_api import AsyncClient
from solana.system_program import SYS_PROGRAM_ID
from incrementer.instructions import create, increment

IML1 = [56,148,44,55,92,31,202,61,172,155,168,205,29,201,251,85,44,146,153,130,45,53,215,32,123,217,163,232,103,25,243,28,88,42,237,121,92,76,242,132,78,37,138,43,165,144,0,84,230,130,170,237,171,132,206,241,123,23,139,51,51,79,104,151]

IML2 = [153,31,111,247,54,72,131,173,86,83,147,99,149,19,193,34,33,168,163,54,121,60,212,35,21,169,200,137,215,254,204,147,175,230,136,112,244,175,75,84,120,46,136,90,208,86,228,223,197,166,39,82,73,9,73,198,159,59,242,77,93,38,17,34]


def create_acc():
    """
    방금 만들어낸 base_acc에 대하여 user_acc가 일정 수수료를 지불하고,
    정식으로 BaseAccount으로 등록하게 됨.
    """

    user_acc = Keypair.from_secret_key(IML2)
    # base acc는 그냥 아무렇게나 만들면됨.
    base_acc = Keypair()

    ix = create({
        'base_account': base_acc.public_key,
        'user': user_acc.public_key,
        'system_program': SYS_PROGRAM_ID
    })
    tx = Transaction().add(ix)

    client = AsyncClient(endpoint='https://api.devnet.solana.com')
    wallet = Wallet(payer=user_acc)

    provider = Provider(
        connection=client,
        wallet=wallet,
    )
    res = asyncio.run(provider.send(tx, [base_acc, user_acc]))
    print(res, type(res))
    print("base_public:", base_acc.public_key)
    print("base_private:", base_acc.secret_key)


def increment_acc():

    user_acc = Keypair.from_secret_key(IML1)

    base_public = PublicKey('C94pmG7VyawDLL7w1pTXRyHog7JxBezAxDwgoteVbtC6')

    ix = increment({
        'base_account': base_public
    })
    tx = Transaction().add(ix)
    client = AsyncClient(endpoint='https://api.devnet.solana.com')
    wallet = Wallet(payer=user_acc)

    provider = Provider(
        connection=client,
        wallet=wallet,
    )
    res = asyncio.run(provider.send(tx))
    print(res, type(res))


if __name__ == '__main__':
    increment_acc()