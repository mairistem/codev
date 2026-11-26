# Clinic booking

The booking service of the Rive-Gauche medical clinic: patients book an
appointment with a practitioner, and get a confirmation through the
clinic's mailer.

    npm install
    npm test
    npm start

Patients give a mobile number when they register (`src/patients.ts`); the
clinic does not send text messages today.

## Open topics

- Text-message reminders: the clinic's management is comparing offers from
  messaging providers; no contract is signed. Ask Claire Dubois
  (operations) before building anything on a provider.
