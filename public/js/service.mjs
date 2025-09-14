export async function loadPage(pageId) {
    const response = await fetch(`/api/page/${pageId}`);
    const jsonData = await response.json();
    return jsonData;
}

export async function createPage(name, parentPageId, content) {
    const createPageRequestBody = {
        // pageId: uuidv4(),
        parentPageId,
        name,
        content
    }
    const response = await fetch(`/api/page`, {
        method: 'POST',
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(createPageRequestBody),
    });
    const jsonData = await response.json();
    return jsonData;
}

export async function loadPageList() {
    const response = await fetch(`/api/pages`);
    const jsonData = await response.json();
    return jsonData;
}

export async function loadBlocks(pageId) {
    const response = await fetch(`/api/page/${pageId}/blocks`);
    const jsonData = await response.json();
    return jsonData;
}

export async function loadUser(username, password){
    let response;
    try{
        response = await fetch('/api/auth/login', {
            method: 'POST',
            headers: {
                "Content-Type": "application/json",
                // 'Content-Type': 'application/x-www-form-urlencoded',
            },
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
            body: JSON.stringify({username, password}),
        });
    }catch(e){ 
        console.error('ERROR LOADING USER', e);
        throw e;
    }

    const jsonData = await response.json();
    if(jsonData && jsonData.status==="success"){
        const result = {...jsonData, token: jsonData.token};
        return result;
    } else {
        console.error('FAILED TO PROCESS USER', jsonData);
        throw new Error("Username or password is incorrect");
    }
}

export async function register(name, email, password){
    let response;
    try{
        response = await fetch('/api/auth/register', {
            method: 'POST',
            headers: {
                "Content-Type": "application/json",
            },
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
            body: JSON.stringify({name, email, password}),
        });
    }catch(e){ 
        console.error('ERROR REGISTERING USER', e);
        throw e;
    }

    const jsonData = await response.json();
    return jsonData;
}

export async function logoutUser(){
    let response;
    try{
        response = await fetch('/api/auth/logout', {
            method: 'POST',
            headers: {
                "Content-Type": "application/json",
            },
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        });
    }catch(e){ 
        console.error('ERROR LOGGING OUT USER', e);
        throw e;
    }
}

export async function saveBlock(blockRequest) {
    const {blockId, pageVersionId} = blockRequest;
    //blockRequest.content = JSON.stringify(blockRequest.content || {});
    const body = JSON.stringify(blockRequest);
    console.info("SAVE BLOCK REQUEST", body);
    let response;
    if(blockId){
        response = await fetch(`/api/page-version/${pageVersionId}/block/${blockId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: body
        });
    }else { 
        response = await fetch(`/api/page-version/${pageVersionId}/block`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: body
        });
    }
    const jsonData = await response.json();
    return jsonData;
}
