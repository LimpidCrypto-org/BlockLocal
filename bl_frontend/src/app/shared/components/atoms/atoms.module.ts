import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BackgroundImageComponent } from './background-image/background-image.component';
import { ButtonComponent } from './button/button.component';
import { CircleComponent } from './circle/circle.component';
import { HeroImageComponent } from './hero-image/hero-image.component';
import { ImageComponent } from './image/image.component';
import { LinkComponent } from './link/link.component';
import { ModalComponent } from './modal/modal.component';
import { Rotate180Component } from './rotate180/rotate180.component';
import { NavigationComponent } from './navigation/navigation.component';
import { FormComponent } from './form/form.component';



@NgModule({
  declarations: [
    BackgroundImageComponent,
    ButtonComponent,
    CircleComponent,
    HeroImageComponent,
    ImageComponent,
    LinkComponent,
    ModalComponent,
    Rotate180Component,
    NavigationComponent,
    FormComponent,
  ],
  imports: [
    CommonModule
  ],
  exports: [
    BackgroundImageComponent,
    ButtonComponent,
    CircleComponent,
    HeroImageComponent,
    ImageComponent,
    LinkComponent,
    ModalComponent,
    Rotate180Component,
    NavigationComponent,
    FormComponent,
  ]
})
export class AtomsModule { }
