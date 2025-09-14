import { createAlert, createModal } from "./modal.mjs";
import { useContext } from "./context.mjs";
import { loadUser, register, logoutUser } from "./service.mjs";

export class UserElement extends HTMLElement {
    static get observedAttributes() {
        return ['user'];
    }

    connectedCallback() {
        const userContext = useContext('user');
        const userString = localStorage.getItem('user');
        if(userString) {
            this.user = JSON.parse(userString);
            console.info(`Loaded user from cache ${this.user.name} with page ${this.user.page_version_id}`);
            userContext.set('user', this.user);
        }
        const shadow = this.attachShadow({ mode: "open" });
        const linkElement = document.createElement("style");
        linkElement.textContent =`
        #login {
            max-height: 1.5rem;
            margin-top: 12px;   /* TODO: Fix alignment */
        }
        .register {
            color: #888;
            letter-spacing: .1rem;
            font-size: 0.8rem;
            /*text-align: center;*/
            cursor: pointer;
        }

        div.login {
            display: flex;
            flex-direction: row;
        }
        div.login > div > span {
            font-size: 8pt;
            font-weight: 300;
            color: #333;
        }
        
        div.login > div{
            display: flex;
            flex-direction: column;
            width: 80px;
            margin-right: 10px;
        }
        `
        shadow.appendChild(linkElement);
        this.shadow = shadow;
        const wrapper =document.createElement('div');
        this.wrapper = wrapper;
        shadow.appendChild(wrapper);
        this.show();
    }

    async loginUser(){
        const userContext = useContext('user');
        const pageContext = useContext('page');

        const username = this.shadow.getElementById('username').value;
        const password = this.shadow.getElementById('password').value;
        
        try{
            const result = await loadUser(username, password);
            if(result && result.user){
                this.user = result.user;
                userContext.set('user', this.user);
                localStorage.setItem('user', JSON.stringify(this.user));
                console.info(`LOADED USER ${this.user.name} WITH PAGE ${this.user.defaultPageId}`);
                if(this.user.defaultPageId) pageContext.set('page', this.user.defaultPageId);
                this.show();
            }
        } catch(e){
            console.error('LOGIN FAILED', e);
            createAlert('error', 'Login Failed', e.message);
        }
    }
    
    async logoutUser(){
        const userContext = useContext('user');
        
        this.user = null;
        userContext.set('user', this.user);
        localStorage.removeItem('user');
        await logoutUser();
    }
    
    show(){
        
        this.wrapper.innerHTML = this.isLoggedin() ? `<div>Welcome: ${this.user.name}&nbsp;&nbsp;&nbsp;<button id="logout" >Logout</button></div>` :
            `<div class="login"><div><span>Username</span><input id="username"></input></div><div><span>Password</span><input type="password" id="password"></input></div><button id="login" >Login</button></div>
            <div id="register" class="register">Register a new user...</div>`;    

        const loginButton = this.shadow.getElementById('login');
        if(loginButton) {
            loginButton.addEventListener('click', () => {
                this.loginUser();
            });
        } else console.error('LOGIN BUTTON NOT FOUND');           

        const registerButton = this.shadow.getElementById('register');
        if(registerButton) {
            registerButton.addEventListener('click', () => {
                this.registerUser();
            });
        }            

        const logoutButton = this.shadow.getElementById('logout');
        if(logoutButton) {
            logoutButton.addEventListener('click', () => {
                this.logoutUser();
                this.show();
            });
        }            
    }

    isLoggedin() {
        return !!this.user;
    }

    async attributeChangedCallback(name, oldValue, newValue) {
        if(name === 'user') {
            this.user = newValue
            this.show();
        }
    }

    registerUser() {
        const body = `
        <div><span>Username: </span><input type="text" id="register_username"/></div>
        <div><span>Email Address: </span><input type="text" id="register_email"/></div>
        <div><span>Password: </span><input type="password" id="register_password1"/></div>
        <div><span>Confirm: </span><input type="password"  id="register_password2"/></div>`
        createModal('Register a new user...', body, ['Register', 'Cancel'], (e,a,c) => this.handleRegisterCallback(e,a,c));
    }

    handleRegisterCallback(e, label, parentElement) {
        if(label === 'Register') {
            const username = parentElement.querySelector('#register_username').value;
            const email = parentElement.querySelector('#register_email').value;
            const password1 = parentElement.querySelector('#register_password1').value;
            const password2 = parentElement.querySelector('#register_password2').value;
            if(password1 !== password2) {
                createAlert('error', 'Password Mismatch', 'The passwords do not match');
                return;
            }
            register(username, email, password1);
        }
    }

    handleItemClick(e, id, dataset) {
        this.classList.toggle('toggle__closed');
        this.dispatchEvent(
            new CustomEvent('action', {
                bubbles: true,
                composed: true,
                detail: {
                    id: id,
                    data: dataset,
                },
            })
        );
    }
}

