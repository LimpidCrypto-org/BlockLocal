import { Component, Input } from '@angular/core';

@Component({
  selector: 'form-atom',
  templateUrl: './form.component.html',
  styleUrl: './form.component.scss'
})
export class FormComponent {
  @Input() target: string | null = null;
}
