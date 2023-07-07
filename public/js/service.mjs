export async function loadPage(pageId) {
    const response = await fetch(`/api/page/${pageId}`);
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

export async function saveBlock(blockRequest) {
    const {blockId, pageVersionId} = blockRequest;
    
    let response;
    if(blockId){
        response = await fetch(`/api/page-version/${pageVersionId}/block/${blockId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(blockRequest)
        });
    }else { 
        response = await fetch(`/api/page-version/${pageVersionId}/block`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(blockRequest)
        });
    }
    const jsonData = await response.json();
    return jsonData;
}
