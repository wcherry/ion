export function createIonBlockElement(id, type, content) {
    //console.log(`ID: ${id}, Type: ${type}, Content: ${content}`)
    const blockElement = document.createElement('ion-block');
    blockElement.id = id;
    blockElement.setAttribute('type', type);
    blockElement.innerHTML = content;
    return blockElement;
}

export class BlockElement extends HTMLElement {
    static get observedAttributes() {
        return ['name', 'type', 'menu' ]
    }
    connectedCallback() {
        const self = this;
        const id = this.attributes.id.value
        const type = this.attributes.type.value
        const body = self.innerHTML;
        self.innerHTML = `<div class="element" data-type="${type}"><menu-toggle></menu-toggle><div class="el editable ${type}" contenteditable="true">${body}</div></div>`;    
    }
    attributeChangedCallback(name, oldValue, newValue) {
        // console.log('Attribute changed', name, oldValue, newValue);
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
    }
    
    setMenu(menuHandler) {
        this.querySelector('.menu-toggle').addEventListener('click', (e) => menuHandler(e, this));
    }
}
