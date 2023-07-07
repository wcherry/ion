import { createAlert, createModal } from "./modal.mjs";
import { useContext } from "./context.mjs";
import { loadUser, register } from "./service.mjs";

export class UserElement extends HTMLElement {
    static get observedAttributes() {
        return ['user'];
    }

    connectedCallback() {
        const userContext = useContext('user');
        const userString = localStorage.getItem('user');
        if(userString) {
            this.user = JSON.parse(userString);
            console.info("Loaded user from cache ${this.user.name} with page ${this.user.page_version_id}");
            userContext.set('user', this.user);
        }
        this.show();
    }

    async loginUser(){
        const userContext = useContext('user');
        const pageContext = useContext('page');
        
    try{
        const result = await loadUser(this.username, this.password);
        if(result && result.user){
            this.user = result.user;
            userContext.set('user', this.user);
            localStorage.setItem('user', JSON.stringify(this.user));
            console.info(`LOADED USER ${this.user.name} WITH PAGE ${this.user.default_page_id}`);
            if(this.user.default_page_id) pageContext.set('page', this.user.default_page_id);
            this.show();
        }
    } catch(e){
        console.error('LOGIN FAILED', e);
        createAlert('error', 'Login Failed', e.message);
    }
}
    
    async logoutUser(){
        const userContext = useContext('user');
        const pageContext = useContext('page');
        
        this.user = null;
        userContext.set('user', this.user);
        localStorage.removeItem('user');
    }
    
    show(){
        this.innerHTML = this.isLoggedin() ? `<div>Username: ${this.user.name} <button id="logout" >Logout</button></div>` :
            `<div class="login"><div><span>Username</span><input id="username"></input></div><div><span>Password</span><input type="password" id="password"></input></div><button id="login" >Login</button></div>
            <div id="register" class="register">Register a new user...</div>`;    

        const loginButton = document.getElementById('login');
        if(loginButton) {
            loginButton.addEventListener('click', () => {
                this.loginUser();
            });
        }            

        const registerButton = document.getElementById('register');
        if(registerButton) {
            registerButton.addEventListener('click', () => {
                this.registerUser();
            });
        }            

        const logoutButton = document.getElementById('logout');
        if(logoutButton) {
            logoutButton.addEventListener('click', () => {
                this.logoutUser();
                this.show();
            });
        }            
        const usernameInput = document.getElementById('username');
        if(usernameInput) {
            usernameInput.addEventListener('change', (e) => {
                this.username = e.target.value;
            });
        }
        const passwordInput = document.getElementById('password');
        if(passwordInput) {
            passwordInput.addEventListener('change', (e) => {              
                this.password = e.target.value;
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

