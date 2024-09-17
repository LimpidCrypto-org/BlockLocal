import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MasterLayout } from './master.component';

describe('MasterLayout', () => {
  let component: MasterLayout;
  let fixture: ComponentFixture<MasterLayout>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [MasterLayout]
    })
      .compileComponents();

    fixture = TestBed.createComponent(MasterLayout);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
