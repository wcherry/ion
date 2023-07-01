export async function loadPage(pageId) {
    const response = await fetch(`/api/page-version/${pageId}/blocks`);
    const jsonData = await response.json();
    return jsonData;
}

export async function loadUser(username, password){
    const response = await fetch('/api/login', {
        method: 'POST',
        headers: {
            "Content-Type": "application/json",
            // 'Content-Type': 'application/x-www-form-urlencoded',
        },
        redirect: "follow", // manual, *follow, error
        referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify({username, password}),
    });
    const jsonData = await response.json();
    return jsonData;
//    return {username: "admin", password: "admin"};
}
/*INPUT:
    pub block_id: Option<String>,
    pub version: Option<i32>,
    pub block_type: String,
    pub content: Option<String>,
    pub display_order: i32,
  OUTPUT:
        id: block.id.to_string(),
        block_id: block.block_id.to_string(),
        version: block.version,
        display_order: page_block_index.display_order,
        block_type: block.block_type,
        content: block.content,
        created_at: block.created_at,
        updated_at: block.updated_at,
        created_by: block.created_by,
        updated_by: block.updated_by,
        active: block.active,
*/
export async function insertBlock(pageVersionId,  blockRequest) {
    const response = await fetch(`/api/page-version/${pageVersionId}/block`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(blockRequest)
    });
    const jsonData = await response.json();
    return jsonData;
}

export async function updateBlock(pageVersionId,  blockRequest) {
    const response = await fetch(`/api/page-version/${pageVersionId}/block`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(blockRequest)
    });
    const jsonData = await response.json();
    return jsonData;
}
