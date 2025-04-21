import { Button } from "~/components/ui/button";

export default function SettingsAccountContent() {
  const handleChangeUsername = () => {
    console.log("Initiate Minecraft username change...");
    // Add your logic here
  };

  return (
    <div>
      <h3 className="text-lg font-medium mb-4">Account Settings</h3>
      <p className="text-sm text-muted-foreground mb-6">
        Manage your account details and connections.
      </p>
      <div className="space-y-4">
        <div>
          <label className="text-sm font-medium block mb-2">
            Minecraft Username
          </label>
          {/* Display current username if available */}
          <Button onClick={handleChangeUsername} variant="outline">
            Change Minecraft Username
          </Button>
        </div>
        {/* Add other account settings elements */}
      </div>
    </div>
  );
}
