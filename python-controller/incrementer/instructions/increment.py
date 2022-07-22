from __future__ import annotations
import typing
from solana.publickey import PublicKey
from solana.transaction import TransactionInstruction, AccountMeta
from ..program_id import PROGRAM_ID


class IncrementAccounts(typing.TypedDict):
    base_account: PublicKey


def increment(accounts: IncrementAccounts) -> TransactionInstruction:
    keys: list[AccountMeta] = [
        AccountMeta(pubkey=accounts["base_account"], is_signer=False, is_writable=True)
    ]
    identifier = b"\x0b\x12h\th\xae;!"
    encoded_args = b""
    data = identifier + encoded_args
    return TransactionInstruction(keys, PROGRAM_ID, data)
