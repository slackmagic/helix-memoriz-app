# ARCH MEMORIZ <> SLED

## BOARDS user ownership:

(K, V) = (USER_ID, BOARDS_ID_VEC)

## List of BOARDS tree:

(K, V) = (BOARD_ID, BOARD_STRUCT)

## Single BOARD tree:

(K, V) = (COMPOUND_ID, ENTRY_STRUCT)

K => (BOARD_ID):(ENTRY_ID)

Permit to make range search
for kv_result in tree.range("key_1".."key_9") {}

## Label to define
