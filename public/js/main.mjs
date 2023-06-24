
import {loadPage, insertBlock} from './service.mjs';
import {createIonBlockElement, BlockElement} from './block.mjs';
import { NavMenuElement, NavMenuItemElement, NavSubMenuElement } from './menu.mjs';

function styleOperation(el, data) {
    const oldType = el.attributes.type.value;
    // console.log('styleOperation', el, data);
    const newType = data.type;
    const id = el.id;

    console.log(`Changing ${id} from ${oldType} to ${newType}`);
    el.setAttribute('type', newType);
}

function copyBlockLinkOperation(el) {
    const id = getState(el.parentElement, 'elementId');
    const blockAddress = document.location.href.split('#')[0] + '#' + id;
    navigator.clipboard.writeText(blockAddress);
    //toast???
}

async function createBlockOperation(el) {
    console.log('createBlockOperation', el);
    const id = el.id;
    const parentElement = el.parentElement;
    const lastType = el.attributes.type.value;
    const index = [...document.querySelector(".body").children].indexOf(el);
    console.log(`Creating new block after ${id} of type ${lastType} at index ${index}`)
    let result = await insertBlock('ea636765-dae1-495e-bda5-a55d74284449', {block_type: lastType, content: 'Sample text #2', display_order: index+2});
    let newBlockElement = createIonBlockElement(result.id, result.block_type, result.content);
    el.after(newBlockElement);
    newBlockElement.setMenu(handleMenuClick);
}

function handleMenuClick(event, blockElement) {
    //console.log('Menu click', event, blockElement);
    const elementMenu = document.querySelector('.menu');
    elementMenu.classList.toggle('toggle__closed');
    elementMenu.dataset.blockId = blockElement.id;
    const type = event.detail.type || 'paragraph';
    elementMenu.dataset.type = type;
    elementMenu.style.top=`${blockElement.getBoundingClientRect().top}px`;
}

document.addEventListener("action", function(event) {
    //console.log('Action event', event.detail);
    const operation = event.detail.data.operation;
    
    const elementMenu = document.querySelector('.menu');
    const blockId = elementMenu.dataset.blockId;
    const block = document.getElementById(blockId);
    //console.log('Action event', event.detail, blockId, block);
    switch(operation) {
        case 'style': styleOperation(block, event.detail.data); break;
        case 'copy_block_link': copyBlockLinkOperation(block); break;
        case 'del': console.log('Delete'); break;
        case 'copy': console.log('Copy'); break;
        case 'dup': console.log('Duplicate'); break;
        case 'create_page': console.log('Create page...'); break;
        case 'create_block': createBlockOperation(block); break;
        default: console.log('Unknown operation', operation);
    }
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

    const event = new Event("action");

    const nav = document.querySelector('.nav__left');
    const elementMenu = document.querySelector('.menu');
    const navToggle = document.querySelector('.nav__toggle');
    const navLinks = document.querySelectorAll('.nav__link');
    const editable = document.querySelectorAll('.editable');

    //const menuToggles = document.querySelectorAll('.menu-toggle');
    //const elementMenuLink = document.querySelectorAll('.menu__link');

    // editable.forEach(el =>{ 
    //     el.addEventListener('blur', () => {
    //         console.log('Saving...',el.parentElement.dataset.id);
    //     });
    //     el.addEventListener('mouseup', () => {
    //         const selection = window.getSelection().toString();
    //         if(selection) console.log('Selection changed...',el.parentElement.dataset.id, selection);
    //     });
    // });

    // menuToggles.forEach(el => { el.addEventListener('click', (e) => {
        //     elementMenu.classList.toggle('menu__open');
        //     const blockElement = el.parentElement.parentElement;
        //     elementMenu.dataset.elementId = blockElement.id
        //     let type = blockElement.attributes.type.value;
        //     console.log('Menu toggle', blockElement.id, type);
        //     elementMenu.querySelectorAll(`.menu__link`).forEach(link=>{link.classList.remove('selected__menu__link')});
        //     elementMenu.querySelector(`.menu__link[data-type="${type}"]`).classList.add('selected__menu__link');
        //     elementMenu.style.top=`${el.getBoundingClientRect().top}px`;
        // });
    // });

    navToggle.addEventListener('click', () => {
        nav.classList.toggle('nav__open');
        navToggle.classList.toggle('toggle__closed');
        navToggle.innerHTML = navToggle.classList.contains('toggle__closed') ? ">>" : "<<";
    });

    // navLinks.forEach(link => {
    //     link.addEventListener('click', () => {
    //         nav.classList.remove('nav__open');
    //     });
    // });

    // elementMenuLink.forEach(link => {link.addEventListener('click', () => {
    //         switch(link.dataset.op) {
    //             case 'style': styleOperation(link); break;
    //             case 'copy_block_link': copyBlockLinkOperation(link); break;
    //             case 'del': console.log('Delete'); break;
    //             case 'copy': console.log('Copy'); break;
    //             case 'dup': console.log('Duplicate'); break;
    //             case 'create_page': console.log('Create page...'); break;
    //             case 'create_block': createBlockOperation(link); break;
    //             default: console.log('Unknown operation', link.dataset.op);
    //         }
    //         elementMenu.classList.toggle('menu__open');
    //     });
    // });

    //Toggle sub-menus on click parent__menu" data-sub-menu="menu_style_sub-list"
    // document.querySelectorAll('.parent__menu').forEach(el => {
    //     el.addEventListener('click', () => {
    //         const subMenu = document.getElementById(el.dataset.subMenu);
    //         subMenu.classList.toggle('toggle__closed');
    //     });
    // });

    (async function() {
        const blocks = await loadPage('ea636765-dae1-495e-bda5-a55d74284449/blocks');        //TODO: lookup pageid from user profile
        console.log('Loaded blocks', blocks);
        const body = document.querySelector('.body');
        blocks.forEach(block => {
            const el = createIonBlockElement(block.id, block.block_type, block.content);
            body.append(el);
            el.setMenu(handleMenuClick);
        });
    })();
});
