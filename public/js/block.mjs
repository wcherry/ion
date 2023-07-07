import { saveBlock } from './service.mjs';

export function createIonBlockElement(block) {
    const blockElement = document.createElement('ion-block');
    const {id, type, content, writable} = block;
    blockElement.id = id;
    blockElement.setAttribute('type', type);
    blockElement.setAttribute('writable', writable);
    blockElement.block = block;
    blockElement.innerHTML = content;
    return blockElement;
}


export class BlockElement extends HTMLElement {
    static get observedAttributes() {
        return ['name', 'type', 'menu', 'writable']
    }

    set block(block) {
        this._block = block;
    }

    isDirty() {
        if(this.dataset.state == "" || this.dataset.state != this.querySelector('.editable').textContent) return true;
    }

    connectedCallback() {
        const self = this;
        const id = this.attributes.id.value;
        const type = this.attributes.type.value;
        const writable = this.attributes.writable ? this.attributes.writable.value : false;
        const body = self.innerHTML;
        // TODO: If user doesn't have permission to edit, remove contenteditable="true"
        self.innerHTML = `<div class="element" data-type="${type}"><menu-toggle></menu-toggle><div class="el editable ${type}" contenteditable="${writable}">${body}</div></div>`;   
        
        const editableElement = this.querySelector(".editable");
        editableElement.addEventListener('keyup', (e) => {
            //TODO: run the markdown parser on the element, and save the result
            if(e.key === 'Enter') {
                e.preventDefault();
                if(self.isDirty()) {
                    saveBlock({...this._block, content:editableElement.textContent});
                }
            }
        });
        editableElement.addEventListener('focus', (e) => {      
            //TODO: record the current state of the element
            this.dataset.state = editableElement.textContent;
        });
        editableElement.addEventListener('blur', (e) => {       
            //TODO: check if the element has changed, if so, save it
            console.info("SAVE BLOCK", this._block);
            if(this.isDirty()) {
                saveBlock({...this._block, content:editableElement.textContent});
            }
        });
    }
    attributeChangedCallback(name, oldValue, newValue) {
        const editableElement = this.querySelector(".editable");
        if(name == 'type' && editableElement) {
            editableElement.classList.remove(oldValue);
            editableElement.classList.add(newValue);
        }
        if(name == 'menu') {
            const menuToggle = this.querySelector('menu-toggle');
            //if(menuToggle) menuToggle.dataset.menu = newValue;

            this.querySelector('.menu-toggle').addEventListener('click', (e) => newValue(e, this));
        }
        if(name == 'writable'  && editableElement) {
            if(newValue == 'true') {
                editableElement.setAttribute('contenteditable', 'true');
            } else {
                editableElement.removeAttribute('contenteditable');
            }
        }
    }
    
    setMenu(menuHandler) {
        this.querySelector('.menu-toggle').addEventListener('click', (e) => menuHandler(e, this));
    }
}
