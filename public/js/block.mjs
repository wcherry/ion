import { saveBlock } from './service.mjs';

export function createIonBlockElement(block) {
    console.log("CREATE ION BLOCK ELEMENT", block);
    const blockElement = document.createElement('ion-block');
    const {id, type, content, writable} = block;
    blockElement.id = id;
    blockElement.setAttribute('type', type);
    blockElement.setAttribute('writable', writable);
    blockElement.block = block;
    blockElement.innerHTML = content;
    // blockElement.setAttribute('contenteditable', writable);
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

    startEdit() {
        let p = this.querySelector('.editable')
        let s = window.getSelection()
        let r = document.createRange()
        console.log("COUNT", p.textContent.length, p.textContent);
        let length= p.textContent.length;
        r.setStart(p, length-2);
        r.setEnd(p, length-2);
        // r.setStart(p, p.childElementCount)
        // r.setEnd(p, p.childElementCount)
        s.removeAllRanges()
        s.addRange(r)
    }

    connectedCallback() {
        const self = this;
        this.setAttribute('tabindex', '0');
        const id = this.attributes.id.value;
        const type = this.attributes.type.value;
        const writable = this.attributes.writable ? this.attributes.writable.value : false;
        const body = self.innerHTML;
        // BUG: If the user doesn't have permission to edit, the contenteditable attribute is still set to true
        // TODO: If user doesn't have permission to edit, remove contenteditable="true"
        self.innerHTML = `<div class="element" data-type="${type}"><menu-toggle></menu-toggle><div class="el editable ${type}" contenteditable="${writable}">${body}</div></div>`;   
        
        const editableElement = this.querySelector(".editable");

        editableElement.addEventListener('keypress', (e) => {
            //FIXME: This is not working
            //TODO: run the markdown parser on the element, and save the result
            if(e.keyCode === 13) {
                console.info("KEYPRESS", e);
                e.preventDefault();
                e.stopPropagation();
                // if(self.isDirty()) {
                //     saveBlock({...this._block, content:editableElement.textContent});
                // }
                const el = createIonBlockElement({type:'paragraph', content:'Sample text #2', writable:true, block: {pageVersionId: this._block.pageVersionId, display_order: this._block.display_order+1}});
                document.querySelector('.body').append(el);
                self.blur();
                //el.focus();
                setTimeout(function() {
                    el.startEdit();
                }, 0);

                return false;
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
