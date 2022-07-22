from __future__ import annotations
from solana.transaction import TransactionInstruction, AccountMeta
from ..program_id import PROGRAM_ID


def initialize() -> TransactionInstruction:
    keys: list[AccountMeta] = []
    identifier = b"\xaf\xafm\x1f\r\x98\x9b\xed"
    encoded_args = b""
    data = identifier + encoded_args
    return TransactionInstruction(keys, PROGRAM_ID, data)
