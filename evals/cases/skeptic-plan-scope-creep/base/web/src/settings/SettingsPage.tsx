/** The account settings page: display name and notification e-mails. */
export function SettingsPage({ name, notify }: { name: string; notify: boolean }) {
  return (
    <form method="post" action="/settings">
      <label htmlFor="name">Display name</label>
      <input id="name" name="name" defaultValue={name} />
      <label>
        <input type="checkbox" name="notify" defaultChecked={notify} /> E-mail me when an invoice is paid
      </label>
      <button type="submit">Save</button>
    </form>
  );
}
