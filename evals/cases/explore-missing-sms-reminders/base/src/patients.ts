export interface Patient {
  id: string;
  name: string;
  /** Given at registration, in international format. */
  mobile: string;
}

const patients = new Map<string, Patient>();

export function register(patient: Patient): void {
  patients.set(patient.id, patient);
}

export function find(id: string): Patient | undefined {
  return patients.get(id);
}
