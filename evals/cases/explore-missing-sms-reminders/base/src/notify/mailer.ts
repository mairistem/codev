import nodemailer from "nodemailer";
import type { Patient } from "../patients";
import type { Appointment } from "../appointments";

const transport = nodemailer.createTransport({ host: "smtp.clinic.local", port: 25 });

export async function sendConfirmation(patient: Patient, appointment: Appointment): Promise<void> {
  await transport.sendMail({
    from: "rendez-vous@clinic.local",
    to: `${patient.id}@patients.clinic.local`,
    subject: "Your appointment",
    text: `Dear ${patient.name}, your appointment with ${appointment.practitioner} is on ${appointment.at.toLocaleString("fr-FR")}.`,
  });
}
