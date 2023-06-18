
function getState(el, name, defaultValue) {
    if(el.dataset[name]) {
        return el.dataset[name];
    } else if(el.parentElement) {
        return getState(el.parentElement, name, defaultValue);
    }
    return defaultValue;
}

function styleOperation(el) {
    const id = getState(el.parentElement, 'elementId');
    const parentElement = document.getElementById(id);
    const oldType = parentElement.attributes.type.value;
    const newType = el.dataset.type;

    console.log(`Changing ${id} from ${oldType} to ${newType}`);
    parentElement.attributes.type.value = newType;
}

function copyBlockLinkOperation(el) {
    const id = getState(el.parentElement, 'elementId');
    const blockAddress = document.location.href.split('#')[0] + '#' + id;
    navigator.clipboard.writeText(blockAddress);
    //toast???
}

document.addEventListener("DOMContentLoaded", function() {
    class MenuToggle extends HTMLElement {
        //    Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->
            connectedCallback() {
                this.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512"><path d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>`;
                this.classList.add('menu-toggle');
            }
    }
    customElements.define('menu-toggle', MenuToggle);
    
    class BlockElement extends HTMLElement {
        static get observedAttributes() {
            return ['name', 'type', ]
        }
        connectedCallback() {
            const self = this;
            const id = this.attributes.id.value
            const type = this.attributes.type.value
            const body = self.innerHTML;
            self.innerHTML = `<div class="element" data-type="${type}"><menu-toggle></menu-toggle><div class="el editable ${type}" contenteditable="true">${body}</div></div>`;    
        }
        attributeChangedCallback(name, oldValue, newValue) {
            console.log('Attribute changed', name, oldValue, newValue);
            const editableElement = this.querySelector(".editable");
            if(name == 'type' && editableElement) {
                editableElement.classList.remove(oldValue);
                editableElement.classList.add(newValue);
            }
        }        
    }
    
    customElements.define('ion-block', BlockElement);

    const nav = document.querySelector('.nav__left');
    const elementMenu = document.querySelector('.menu');
    const navToggle = document.querySelector('.nav__toggle');
    const navLinks = document.querySelectorAll('.nav__link');
    const editable = document.querySelectorAll('.editable');
    const menuToggles = document.querySelectorAll('.menu-toggle');
    const elementMenuLink = document.querySelectorAll('.menu__link');

    editable.forEach(el =>{ 
        el.addEventListener('blur', () => {
            console.log('Saving...',el.parentElement.dataset.id);
        });
        el.addEventListener('mouseup', () => {
            const selection = window.getSelection().toString();
            if(selection) console.log('Selection changed...',el.parentElement.dataset.id, selection);
        });
    });

    menuToggles.forEach(el => { el.addEventListener('click', (e) => {
            elementMenu.classList.toggle('menu__open');
            const blockElement = el.parentElement.parentElement;
            elementMenu.dataset.elementId = blockElement.id
            let type = blockElement.attributes.type.value;
            console.log('Menu toggle', blockElement.id, type);
            elementMenu.querySelectorAll(`.menu__link`).forEach(link=>{link.classList.remove('selected__menu__link')});
            elementMenu.querySelector(`.menu__link[data-type="${type}"]`).classList.add('selected__menu__link');
            elementMenu.style.top=`${el.getBoundingClientRect().top}px`;
        });
    });

    navToggle.addEventListener('click', () => {
        nav.classList.toggle('nav__open');
        navToggle.classList.toggle('nav__toggle__open');
        navToggle.innerHTML = navToggle.classList.contains('nav__toggle__open') ? ">>" : "<<";
    });

    navLinks.forEach(link => {
        link.addEventListener('click', () => {
            nav.classList.remove('nav__open');
        });
    });

    elementMenuLink.forEach(link => {link.addEventListener('click', () => {
            switch(link.dataset.op) {
                case 'style': styleOperation(link); break;
                case 'copy_block_link': copyBlockLinkOperation(link); break;
                case 'del': console.log('Delete'); break;
                case 'copy': console.log('Copy'); break;
                case 'dup': console.log('Duplicate'); break;
                case 'create_page': console.log('Create page...'); break;
                case 'create_block': console.log('Create block...'); break;
                default: console.log('Unknown operation', link.dataset.op);
            }
            elementMenu.classList.toggle('menu__open');
        });
    });

    //Toggle sub-menus on click parent__menu" data-sub-menu="menu_style_sub-list"
    document.querySelectorAll('.parent__menu').forEach(el => {
        el.addEventListener('click', () => {
            const subMenu = document.getElementById(el.dataset.subMenu);
            subMenu.classList.toggle('toggle__closed');
        });
    });
});
