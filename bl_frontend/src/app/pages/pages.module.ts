import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DocumentationPage } from './documentation/documentation.component';
import { HomePage } from './home/home.component';
import { SharedModule } from '../shared/shared.module';
import { LoginPage } from './login/login.page';



@NgModule({
    declarations: [
        DocumentationPage,
        HomePage,
        LoginPage,
    ],
    imports: [
        CommonModule,
        SharedModule
    ],
    exports: [
        DocumentationPage,
        HomePage,
        LoginPage,
    ]
})
export class PagesModule { }
