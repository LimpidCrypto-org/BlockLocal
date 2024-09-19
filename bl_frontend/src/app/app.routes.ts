import { Routes } from '@angular/router';
import { HomePage } from './pages/home/home.component';
import { LoginPage } from './pages/login/login.page';
// import { DocumentationPage } from './pages/documentation/documentation.component';

export const routes: Routes = [
    { path: '', component: HomePage },
    { path: 'login', component: LoginPage }
    // { path: 'documentation', component: DocumentationPage },
];
