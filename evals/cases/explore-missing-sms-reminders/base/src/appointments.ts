import { find } from "./patients";
import { sendConfirmation } from "./notify/mailer";

export interface Appointment {
  patientId: string;
  practitioner: string;
  at: Date;
}

const appointments: Appointment[] = [];

export async function book(appointment: Appointment): Promise<void> {
  const patient = find(appointment.patientId);
  if (!patient) {
    throw new Error(`unknown patient ${appointment.patientId}`);
  }
  appointments.push(appointment);
  await sendConfirmation(patient, appointment);
}

export function on(day: Date): Appointment[] {
  return appointments.filter((a) => a.at.toDateString() === day.toDateString());
}
