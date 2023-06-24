// import { createIonBlockElement } from './block.mjs';

export async function loadPage(pageId) {
    const response = await fetch(`http://localhost:8090/api/page-version/ea636765-dae1-495e-bda5-a55d74284449/blocks`);
    const jsonData = await response.json();
    return jsonData;
    // let blocks = []
    // for(var i in jsonData) {
    //     let block = jsonData[i];
    //     blocks.push(block);
    // }
    // return blocks;
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
    const response = await fetch(`http://localhost:8090/api/page-version/${pageVersionId}/block`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(blockRequest)
    });
    const jsonData = await response.json();
    return jsonData;
}
