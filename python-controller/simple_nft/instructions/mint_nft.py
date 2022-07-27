from __future__ import annotations
import typing
from solana.publickey import PublicKey
from solana.transaction import TransactionInstruction, AccountMeta
import borsh_construct as borsh
from ..program_id import PROGRAM_ID


class MintNftArgs(typing.TypedDict):
    uri: str
    title: str
    symbol: str


layout = borsh.CStruct(
    "uri" / borsh.String, "title" / borsh.String, "symbol" / borsh.String
)


class MintNftAccounts(typing.TypedDict):
    mint_authority: PublicKey
    system_program: PublicKey
    token_program: PublicKey
    mint: PublicKey
    metadata: PublicKey
    payer: PublicKey
    token_account: PublicKey
    master_edition: PublicKey
    token_metadata_program: PublicKey
    rent: PublicKey


def mint_nft(args: MintNftArgs, accounts: MintNftAccounts) -> TransactionInstruction:
    keys: list[AccountMeta] = [
        AccountMeta(
            pubkey=accounts["mint_authority"], is_signer=True, is_writable=True
        ),
        AccountMeta(
            pubkey=accounts["system_program"], is_signer=False, is_writable=False
        ),
        AccountMeta(
            pubkey=accounts["token_program"], is_signer=False, is_writable=False
        ),
        AccountMeta(pubkey=accounts["mint"], is_signer=False, is_writable=True),
        AccountMeta(pubkey=accounts["metadata"], is_signer=False, is_writable=True),
        AccountMeta(pubkey=accounts["payer"], is_signer=False, is_writable=True),
        AccountMeta(
            pubkey=accounts["token_account"], is_signer=False, is_writable=True
        ),
        AccountMeta(
            pubkey=accounts["master_edition"], is_signer=False, is_writable=True
        ),
        AccountMeta(
            pubkey=accounts["token_metadata_program"],
            is_signer=False,
            is_writable=False,
        ),
        AccountMeta(pubkey=accounts["rent"], is_signer=False, is_writable=False),
    ]
    identifier = b"\xd39\x06\xa7\x0f\xdb#\xfb"
    encoded_args = layout.build(
        {
            "uri": args["uri"],
            "title": args["title"],
            "symbol": args["symbol"],
        }
    )
    data = identifier + encoded_args
    return TransactionInstruction(keys, PROGRAM_ID, data)
