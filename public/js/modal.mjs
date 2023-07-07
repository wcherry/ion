export class ModalElement extends HTMLElement {
    static get observedAttributes() {
        return ['type']
    }

    constructor() {
        super();
    }

    set handler(handler) {
        this._handler = handler;
    }

    connectedCallback() {
        const type = this.attributes.type.value;
        const subType = this.attributes.subType ? this.attributes.subType.value : 'modal';
        const title = this.attributes.title.value;
        const buttons = this.attributes.buttons ? this.attributes.buttons.value : 'Close'
        const shadow = this.attachShadow({ mode: "open" });
        const linkElement = document.createElement("link");
        linkElement.setAttribute("rel", "stylesheet");
        linkElement.setAttribute("href", "css/alert.css");
        shadow.appendChild(linkElement);
        const wrapper = document.createElement('div');
        wrapper.innerHTML =
        `<div class="shadow" onClick="(e)=>{e.preventDefault(); e.stopPropagation();}"><div class="alert" data-type="${subType}">
        <div class="title"><span>${title}</span><button class="button">X</button></div>
        <div id="body" class="body">${this.innerHtml}</div>
        <div class="buttons"></div>
        </div></div>`;

        const buttonsElement = wrapper.querySelector('.buttons');
        buttons.split(",").forEach(it => {
            const button = document.createElement('button');
            button.classList.add('button');
            button.innerText = it;
            buttonsElement.appendChild(button);
            button.addEventListener('click', (e) => this.handleClose(e, it));
        });

        shadow.appendChild(wrapper);
    }

    handleClose(e, label) {
        if(this._handler) this._handler(e, label, this.shadowRoot.getElementById('body'));
        document.body.removeChild(this);
    }
}

export function createAlert(type, title, message, closeButtonText) {
    const alert = document.createElement('ion-modal');
    alert.setAttribute('type', 'alert');
    alert.setAttribute('subType', type);
    alert.setAttribute('title', title);
    alert.innerHtml = `<span>${message}</span>`;
    document.body.appendChild(alert);
    return alert;
}

export function createModal(title, body, buttons, handler) {
    const modal = document.createElement('ion-modal');
    modal.setAttribute('type', 'modal');
    modal.setAttribute('title', title);
    modal.setAttribute('buttons', buttons);
    modal.handler = handler ? handler : () => {};
    modal.innerHtml = body;
    document.body.appendChild(modal);
    return modal;
}
