export class AlertElement extends HTMLElement {
    static get observedAttributes() {
        return ['type']
    }

    constructor() {
        super();
    }

    connectedCallback() {
        const type = this.attributes.type.value;
        const message = this.attributes.message.value;
        const title = this.attributes.title.value;
        const closeButtonTitle = this.attributes.closeButtonText ?  this.attributes.closeButtonText.value : 'Close';
        this.innerHTML = `<div class="alert" data-type="${type}">
        <div class="title"><span>${title}</span><button class="close">X</button></div>
        <span class="message">${message}</span>
        <div class="buttons"><button class="close">${closeButtonTitle}</button></div>
        </div>`;
        
        this.querySelectorAll('.close').forEach(it=>it.addEventListener('click', () => this.handleClose()));
    }

    handleClose() {
        console.log('handleClose');
        document.body.removeChild(this);
    }
}

export function createAlert(type, title, message, closeButtonText) {
    const alert = document.createElement('ion-alert');
    alert.setAttribute('type', type);
    alert.setAttribute('title', title);
    alert.setAttribute('message', message);
    if(closeButtonText) alert.setAttribute('closeButtonText', closeButtonText);
    document.body.appendChild(alert);
    return alert;
}
