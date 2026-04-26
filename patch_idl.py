import json

path = "patched-arcium-client/idls/arcium.json"
with open(path, "r") as f:
    idl = json.load(f)

# Accounts to keep for on-chain compatibility in PrivaSee
keep_accounts = [
    'ArxNode', 'ClockAccount', 'Cluster', 'ComputationAccount', 
    'ComputationDefinitionAccount', 'ComputationDefinitionRaw', 
    'FailureClaimAccountHeader', 'FeePool', 'MXEAccount', 
    'MxeRecoveryAccount', 'Operator', 'RecoveryClusterAccount', 
    'RecoveryPeerAccount'
]

# Types to remove (the huge ones)
remove_types = [
    'LargeExecPool', 'LargeMempool', 'MediumExecPool', 'MediumMempool', 
    'SmallExecPool', 'SmallMempool', 'TinyExecPool', 'TinyMempool',
    'LargeMempoolInner', 'LargeMempoolInnerBuffer', 'LargeMempoolInnerBufferHeap',
    'MediumMempoolInner', 'MediumMempoolInnerBuffer', 'MediumMempoolInnerBufferHeap',
    'SmallMempoolInner', 'SmallMempoolInnerBuffer', 'SmallMempoolInnerBufferHeap',
    'TinyMempoolInner', 'TinyMempoolInnerBuffer', 'TinyMempoolInnerBufferHeap',
    'LargeExecPoolInner', 'LargeExecPoolInnerBuffer', 'LargeExecPoolInnerBufferHeap',
    'MediumExecPoolInner', 'MediumExecPoolInnerBuffer', 'MediumExecPoolInnerBufferHeap',
    'SmallExecPoolInner', 'SmallExecPoolInnerBuffer', 'SmallExecPoolInnerBufferHeap',
    'TinyExecPoolInner', 'TinyExecPoolInnerBuffer', 'TinyExecPoolInnerBufferHeap',
]

idl['accounts'] = [a for a in idl['accounts'] if a['name'] in keep_accounts]
idl['types'] = [t for t in idl.get('types', []) if t['name'] not in remove_types]

print(f"Purged large types and accounts from IDL.")

# Restoration map based on standard Arcium/Solana types
restoration = {
    "x25519_pubkey": 32,
    "ed25519_verifying_key": 32,
    "elgamal_pubkey": 32,
    "pubkey_validity_proof": 64,
    "bls_sig": 64,
    "peer_id": 32,
    "hash": 32,
    "key_material_hash": 32,
    "cu_price_proposals": 32,
    "valid_bits": 23
}

def patch_fields(fields, parent_name):
    for field in fields:
        if not isinstance(field, dict): continue
        field_name = field.get("name")
        field_type = field.get("type")
        if isinstance(field_type, dict) and "array" in field_type:
            arr = field_type["array"]
            if isinstance(arr, list) and len(arr) == 2:
                if field_name in restoration:
                    field_type["array"][1] = restoration[field_name]
                else:
                    size = arr[1]
                    if isinstance(size, int) and size > 128:
                        print(f"Shrinking large array in {parent_name}.{field_name}: {size} -> 1")
                        field_type["array"][1] = 1

for type_def in idl.get("types", []):
    if type_def.get("type", {}).get("kind") == "struct":
        patch_fields(type_def["type"].get("fields", []), type_def["name"])
                    
for account in idl.get("accounts", []):
    if account.get("type", {}).get("kind") == "struct":
        patch_fields(account["type"].get("fields", []), account["name"])

with open(path, "w") as f:
    json.dump(idl, f, indent=2)

print("IDL fully sanitized.")
