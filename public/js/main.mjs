
import { loadPage, loadBlocks, createPage, loadPageList } from './service.mjs';
import { BlockElement } from './block.mjs';
import { NavMenuElement, NavMenuItemElement, NavSubMenuElement } from './menu.mjs';
import { ContextElement, useContext } from './context.mjs';
import { UserElement } from './user.mjs';
import { ModalElement, createModal } from './modal.mjs';

const DEFAULT_PAGE_ID = '66dd25a9-01ca-47ee-a558-31346e25ab8d';

function styleOperation(el, data) {
    const oldType = el.attributes.type.value;
    const newType = data.type;
    const id = el.id;
    el.setAttribute('type', newType);
}

function createTableOperation(el, data) {
    const blockElement = BlockElement.new({blockType: 'table', content: data});
    el.after(blockElement);
    blockElement.setMenu(handleMenuClick);

    // const oldType = el.attributes.type.value;
    // const {type, cols, rows, style} = data;
    // const id = el.id;
    // el.setAttribute('type', type);
    // let table = `<table class="${style}">`;
    // if(data.headers && data.headers === 'true') {
    //     table += '<thead><tr>';
    //     for(let c=0; c<cols; c++) {
    //         table += `<th class="editable" contenteditable>Header ${c+1}</th>`;
    //     }
    //     table += '</tr></thead>';
    // }
    // table += '<tbody>';
    // for(let r=0; r<rows; r++) {
    //     table += '<tr>';
    //     for(let c=0; c<cols; c++) {
    //         table += `<td class="editable" contenteditable>Data ${r+1}-${c+1}</td>`;
    //     }
    //     table += '</tr>';
    // }
    // table += '</tbody></table>';
    // el.innerHTML = table;
    // el.dataset.dirty = false;
    // const parent = el;
    // parent.querySelectorAll('.editable').forEach(el => {
    //     // el.attributes.contenteditable = true;
    //     el.dataset.value = el.innerHTML;
    //     el.addEventListener('blur', (e) => {
    //         if(el.innerHTML !== el.dataset.value) {
    //             console.info("TABLE CELL CHANGED", el.innerHTML, el.dataset.value);
    //             parent.setAttribute("dirty", true);
    //         }
    //     });
    // });
}

function copyBlockLinkOperation(el) {
    const id = getState(el.parentElement, 'elementId');
    const blockAddress = document.location.href.split('#')[0] + '#' + id;
    navigator.clipboard.writeText(blockAddress);
    //toast???
}

async function createBlockOperation(el) {
    const {pageVersionId} = useContext("page").get("page");
    const index = [...document.querySelector(".body").children].indexOf(el);

    const displayOrder = el.block.displayOrder || 0;

    const newBlockElement = BlockElement.new({pageVersionId, displayOrder: displayOrder+1, blockType: "paragraph", content: `Sample text #${index}`}); //createIonBlockElement(result.id, result.blockType, content);
    el.after(newBlockElement);
    newBlockElement.saveBlock();
    newBlockElement.setMenu(handleMenuClick);
}

async function createPageOperation(el) {
    // Display modal to accept page name
    const body = `<div><span>Page Name: </span><input type="text" id="create_page_name"/></div>`
    createModal('Create new page...', body, ['Create', 'Cancel'], (event, label, parentElement) => {
        if(label === 'Create') {
            const pageName = parentElement.querySelector('#create_page_name').value;
            (async () => {
                // Create page
                const page = useContext("page").get("page");
                const response = await createPage(pageName, page.id, `# ${pageName}`);
                // Display page
                displayPage(response.id);
            })();
        }
    });
}

function handleMenuClick(event, blockElement) {
    const elementMenu = document.querySelector('.menu');
    elementMenu.dataset.blockId = blockElement.id;
    const type = event.detail.type || 'paragraph';
    elementMenu.dataset.type = type;
    elementMenu.style.top=`${blockElement.getBoundingClientRect().top+2}px`;
    elementMenu.style.left=`${blockElement.getBoundingClientRect().left-156}px`;
    elementMenu.style.zIndex=1000;
    elementMenu.classList.toggle('toggle__closed');
}

function handleUserChange(event) {
    const user = event.detail.data.user;
    console.info('USER CHANGED', user);
    if(user){
        displayPage(user.defaultPageId || DEFAULT_PAGE_ID);
        (async () => {
            loadNavMenu();
        })();
    } else {
        displayPage(DEFAULT_PAGE_ID);
    }
}

async function loadNavMenu() {
    const pageList =  await loadPageList();
    const navMenu = document.getElementById('my_pages');
    navMenu.innerHTML = '';
    pageList.forEach(page => {
        const el = document.createElement('ion-menu-item');
        el.innerHTML = `<li>${page.name}</li>`;
        el.dataset.pageId = page.id;
        el.addEventListener('click', () => {
            displayPage(page.id);
        });
        navMenu.append(el);
    });
}

async function displayPage(pageId) {
    const page = await loadPage(pageId);
    useContext("page").set("page", page);

    const blocks = await loadBlocks(pageId);
    const body = document.querySelector('.body');
    body.innerHTML = '';
    blocks.forEach(block => {
        const writable = block.modes.indexOf('owner')>=0 || block.modes.indexOf('admin')>=0 || block.modes.indexOf('write')>=0;
        block.writable = writable;
        block.pageVersionId = page.pageVersionId;
        const el = BlockElement.load(block); // createIonBlockElement(block);
        body.append(el);
        el.setMenu(handleMenuClick);
    });
}

document.addEventListener("action", function(event) {
    const operation = event.detail.data.operation;
    const elementMenu = document.querySelector('.menu');
    const blockId = elementMenu.dataset.blockId;
    const block = document.getElementById(blockId);
    switch(operation) {
        case 'style': styleOperation(block, event.detail.data); break;
        case 'table': createTableOperation(block, event.detail.data); break;
        case 'copy_block_link': copyBlockLinkOperation(block); break;
        case 'del': console.debug('Delete'); break;
        case 'copy': console.debug('Copy'); break;
        case 'dup': console.debug('Duplicate'); break;
        case 'create_page': createPageOperation(block); break;
        case 'create_block': createBlockOperation(block); break;
        default: console.error('Unknown operation', operation);
    }
});

document.addEventListener("context", function(event) {
    if(event.detail.name === 'user') {
        handleUserChange(event);
    }
    // if(event.detail.name === 'page') {
    //     (async () => {
    //         await displayPage(event.detail.data.page.id);
    //     })();
    // }
});

document.addEventListener("DOMContentLoaded", function() {
    class MenuToggle extends HTMLElement {
        //    Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->
            connectedCallback() {
                this.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512"><path d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>`;
                this.classList.add('menu-toggle');
            }
    }
    customElements.define('menu-toggle', MenuToggle);
    
    customElements.define('ion-block', BlockElement);
    customElements.define('ion-floating-menu', NavMenuElement);
    customElements.define('ion-menu-item', NavMenuItemElement);
    customElements.define('ion-sub-menu', NavSubMenuElement);
    customElements.define('ion-context', ContextElement);
    customElements.define('ion-user', UserElement);
    customElements.define('ion-modal', ModalElement);

    const event = new Event("action");
    const context = new Event("context");

    const nav = document.querySelector('.nav__left');
    const navToggle = document.querySelector('.nav__toggle');

    navToggle.addEventListener('click', () => {
        nav.classList.toggle('nav__open');
        navToggle.classList.toggle('toggle__open');
        navToggle.innerHTML = navToggle.classList.contains('toggle__open') ? "<<" : ">>";
    });

    (async function() {
        const user = useContext("user").get("user");
        let pageId = DEFAULT_PAGE_ID;
        if(user){
            pageId = user.defaultPageId; // || DEFAULT_PAGE_ID;
        }
        
        (async () =>{
            await displayPage(pageId);
        })();

        loadNavMenu();

        // document.querySelectorAll('ion-block[type="code"]').forEach(el => {
        //     hljs.highlightElement(el);
        //   })
    })();
});
