import asyncio
from anchorpy import Provider, Wallet
from solana.keypair import Keypair
from solana.publickey import PublicKey
from solana.transaction import Transaction
from solana.rpc.async_api import AsyncClient
from solana.system_program import SYS_PROGRAM_ID
from solana.sysvar import SYSVAR_RENT_PUBKEY
from solana.rpc.core import RPCException
from pprint import pprint

from simple_nft.instructions import mint_nft, MintNftArgs, MintNftAccounts


TOKEN_PROGRAM_ID = PublicKey(
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
TOKEN_METADATA_PROGRAM_ID = PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")


IML1 = [56,148,44,55,92,31,202,61,172,155,168,205,29,201,251,85,44,146,153,130,45,53,215,32,123,217,163,232,103,25,243,28,88,42,237,121,92,76,242,132,78,37,138,43,165,144,0,84,230,130,170,237,171,132,206,241,123,23,139,51,51,79,104,151]

IML2 = [153,31,111,247,54,72,131,173,86,83,147,99,149,19,193,34,33,168,163,54,121,60,212,35,21,169,200,137,215,254,204,147,175,230,136,112,244,175,75,84,120,46,136,90,208,86,228,223,197,166,39,82,73,9,73,198,159,59,242,77,93,38,17,34]

URL = "https://avatars.githubusercontent.com/u/94878956?s=400&u=3f41e32898e3bca65af95438737f79e71d033f05&v=4"

IML_TOKEN = PublicKey("E9S62tvek56rhSjwVVQnEACZg5BEztpTNeZLNv1iGA9Q")
IML1_TOKEN_ACC = PublicKey("D13rtz5bwtv9k8SuPfAWvhsvnQECZnXz5KBhuJuKHdoS")
IML2_TOKEN_ACC = PublicKey("4yeC7em3yjmUHkMJGDMeAihfLiLcfTs9pdqeccFCZngJ")


def main():

    iml1 = Keypair.from_secret_key(IML1)
    iml2 = Keypair.from_secret_key(IML2)
    print("Pub:", iml2.public_key)

    ix = mint_nft(
        MintNftArgs(
            creator_key=IML_TOKEN,
            uri=URL,
            title="IML NFT",
            symbol="IMIML",
        ),
        MintNftAccounts(
            mint_authority=iml2.public_key,
            system_program=SYS_PROGRAM_ID,
            token_program=TOKEN_PROGRAM_ID,
            mint=IML_TOKEN,
            metadata=Keypair().public_key, # How to make...?
            payer=iml2.public_key,
            token_account=IML2_TOKEN_ACC, 
            master_edition=Keypair().public_key, # How to make...?
            token_metadata_program=TOKEN_METADATA_PROGRAM_ID,
            rent=SYSVAR_RENT_PUBKEY,
        )
    )
    tx = Transaction().add(ix)
    client = AsyncClient(endpoint='https://api.devnet.solana.com')
    wallet = Wallet(payer=iml2)
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

"""
{'code': -32002,
 'data': {'accounts': None,
          'err': {'InstructionError': [0, {'Custom': 5}]},
          'logs': ['Program 4jvMtVYUSyWoVvaxwAnWzSNdqaZHJArKG3zyFQGZjuty '
                   'invoke [1]',
                   'Program log: Instruction: MintNft',
                   'Program log: Initializing Mint NFT',
                   'Program log: CPI(Token Program) Account Assigned',
                   'Program log: Token Program Assigned',
                   'Program log: Token Program Context Assigned',
                   'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke '
                   '[2]',
                   'Program log: Instruction: MintTo',
                   'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA '
                   'consumed 2928 of 189584 compute units',
                   'Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA '
                   'success',
                   'Program log: Token Minted !!!',
                   'Program log: Account Info Assgined',
                   'Program log: Creator Assigned',
                   'Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s invoke '
                   '[2]',
                   'Program log: Instruction: Create Metadata Accounts v2',
                   "Program log:  Metadata's key must match seed of "
                   "['metadata', program id, mint] provided",
                   'Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s '
                   'consumed 9687 of 181508 compute units',
                   'Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s '
                   'failed: custom program error: 0x5',
                   'Program 4jvMtVYUSyWoVvaxwAnWzSNdqaZHJArKG3zyFQGZjuty '
                   'consumed 28179 of 200000 compute units',
                   'Program 4jvMtVYUSyWoVvaxwAnWzSNdqaZHJArKG3zyFQGZjuty '
                   'failed: custom program error: 0x5'],
          'unitsConsumed': 0},
 'message': 'Transaction simulation failed: Error processing Instruction 0: '
            'custom program error: 0x5'}
"""