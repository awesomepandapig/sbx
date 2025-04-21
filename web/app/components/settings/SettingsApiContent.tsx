import { useState, useEffect } from "react";
import { Copy, Check, AlertCircle } from "lucide-react"; // Added AlertCircle for errors
import { Button } from "~/components/ui/button"; // Assuming path is correct
import { API_URL } from "~/lib/config"; // Assuming path is correct

// Basic styling for the input-like div (Tailwind assumed)
const inputStyleBase = "p-3 border border-gray-700 bg-gray-900 rounded-md text-sm font-mono whitespace-pre-wrap break-words w-full";
const inputStyleEmpty = `${inputStyleBase} text-muted-foreground`; // Style for empty state
const inputStyleFilled = `${inputStyleBase} text-gray-400`; // Style for stars or key

export default function SettingsApiContent() {
  // State
  const [apiKeyId, setApiKeyId] = useState<string | null>(null); // ID of the current key
  const [apiKeyDisplay, setApiKeyDisplay] = useState<string>(""); // What to show: "", "******", "actual-key"
  const [isNewlyGenerated, setIsNewlyGenerated] = useState<boolean>(false); // Is the displayed value a *new* key?
  const [isLoading, setIsLoading] = useState<boolean>(true); // Start loading for initial fetch
  const [copied, setCopied] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // --- API Functions ---

  // DELETE Key Function (adapted from your code)
  async function deleteKey(keyIdToDelete: string): Promise<boolean> {
    setError(null); // Clear previous errors before new operation
    try {
      console.log(`Attempting to delete key ID: ${keyIdToDelete}`);
      const response = await fetch(`${API_URL}/auth/api-key/delete`, {
        method: "DELETE",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ keyId: keyIdToDelete }), // Ensure body matches API expectation
      });
      if (!response.ok) {
         const errorText = await response.text(); // Try to get more error info
         console.error("Delete Key Response Error:", response.status, errorText);
         throw new Error(`Failed to delete API key (Status: ${response.status}). ${errorText || ""}`);
      }
      console.log(`Successfully deleted key ID: ${keyIdToDelete}`);
      setApiKeyId(null); // Clear the ID state after successful deletion
      return true; // Indicate success
    } catch (err: any) {
      console.error("Error deleting key:", err);
      setError(err.message || "An unexpected error occurred during key deletion.");
      // Do not clear state here, let the calling function decide based on context
      return false; // Indicate failure
    }
  }

  // CREATE Key Function (adapted from your code)
  async function createKey(): Promise<boolean> {
     setError(null); // Clear previous errors
     try {
       console.log("Attempting to create new key...");
       const response = await fetch(`${API_URL}/auth/api-key/create`, {
         method: "POST",
         credentials: "include",
         headers: {
           "Content-Type": "application/json",
         },
         body: JSON.stringify({}), // Empty body as per your original function
       });

       if (!response.ok) {
         const errorText = await response.text();
         console.error("Create Key Response Error:", response.status, errorText);
         throw new Error(`Failed to create API key (Status: ${response.status}). ${errorText || ""}`);
       }

       const data = await response.json();

       if (!data.id || !data.key) {
         console.error("Create Key Response Missing Data:", data);
         throw new Error("API response did not contain the expected key ID and key value.");
       }

       console.log(`Successfully created key ID: ${data.id}`);
       setApiKeyId(data.id);       // Store the new key ID
       setApiKeyDisplay(data.key); // Display the new key value
       setIsNewlyGenerated(true);  // Mark it as newly generated
       return true; // Indicate success

     } catch (err: any) {
       console.error("Error creating key:", err);
       setError(err.message || "An unexpected error occurred during key creation.");
       // If creation fails, reset display state appropriately
       setApiKeyDisplay(""); // Show empty input on creation failure
       setIsNewlyGenerated(false);
       setApiKeyId(null); // Ensure ID is null if creation fails
       return false; // Indicate failure
     }
  }

  // --- Effects ---

  // Initial Key Fetch on Load
  useEffect(() => {
    async function getKey() {
      setIsLoading(true);
      setError(null);
      try {
        console.log("Fetching existing API keys...");
        const response = await fetch(`${API_URL}/auth/api-key/list`, {
          credentials: "include",
        });
        if (!response.ok) {
          const errorText = await response.text();
          console.error("List Keys Response Error:", response.status, errorText);
          throw new Error(`Failed to fetch API key status (Status: ${response.status}). ${errorText || ""}`);
        }
        const data = await response.json();

        if (Array.isArray(data) && data.length > 0 && data[0].id) {
           console.log(`Found existing key ID: ${data[0].id}`);
           setApiKeyId(data[0].id);           // Store the existing key ID
           setApiKeyDisplay("*".repeat(64)); // Show stars for existing key
           setIsNewlyGenerated(false);       // It's not newly generated
        } else {
           console.log("No existing API key found.");
           setApiKeyId(null);           // No key exists
           setApiKeyDisplay("");        // Show empty input
           setIsNewlyGenerated(false);
        }
      } catch (err: any) {
        console.error("Error fetching initial key status:", err);
        setError(err.message || "Could not verify API key status.");
        setApiKeyId(null); // Assume no key if status check fails
        setApiKeyDisplay("");
        setIsNewlyGenerated(false);
      } finally {
        setIsLoading(false);
      }
    }
    getKey();
  }, []); // Run only once on mount

  // --- Handlers ---

  // Combined Delete then Create Handler
  const handleGenerateNewKey = async () => {
    setIsLoading(true);
    setError(null);
    setCopied(false); // Reset copy status
    // Reset visual state immediately *unless* we keep stars until success
    // setApiKeyDisplay("Generating..."); // Optional: indicate processing

    let deleteSuccess = true; // Assume success if no key exists to delete

    // Step 1: Delete existing key if it exists
    if (apiKeyId) {
      console.log("Existing key found. Attempting deletion first...");
      deleteSuccess = await deleteKey(apiKeyId);
    } else {
       console.log("No existing key found. Proceeding to create...");
    }

    // Step 2: Create a new key ONLY if deletion was successful (or not needed)
    if (deleteSuccess) {
      console.log("Deletion successful (or not needed). Attempting creation...");
      await createKey(); // createKey handles setting state on success/failure
    } else {
      console.log("Deletion failed. Aborting key creation.");
      // Error state should already be set by deleteKey failure
      // Revert display to stars if deletion failed but a key *did* exist initially
      if (apiKeyId) { // Check if there *was* an ID before deletion attempt failed
         setApiKeyDisplay("*".repeat(64));
      } else {
         setApiKeyDisplay(""); // Should not happen if apiKeyId was null, but for safety
      }
      setIsNewlyGenerated(false);
    }

    setIsLoading(false);
  };

  // Copy Handler
  const handleCopy = async () => {
    // Only copy if it's the newly generated key visible
    if (!isNewlyGenerated || !apiKeyDisplay || apiKeyDisplay === "*".repeat(64)) return;

    try {
      await navigator.clipboard.writeText(apiKeyDisplay);
      setCopied(true);
      setTimeout(() => setCopied(false), 2500); // Show feedback for 2.5 seconds
    } catch (err) {
      console.error("Copy failed:", err);
      setError("Failed to copy key to clipboard.");
      setCopied(false); // Ensure copied state is reset on error
    }
  };

  // --- Render ---

  // Determine button text based on whether a key currently exists (before generation starts)
  const buttonText = apiKeyId ? "Generate New Key" : "Create API Key";

  // Determine input style and content
  let currentInputStyle = inputStyleFilled;
  let displayValue = apiKeyDisplay;

  if (isLoading && !apiKeyId && !isNewlyGenerated) { // Initial load state
      currentInputStyle = inputStyleEmpty;
  } else if (!isLoading && !apiKeyId && !isNewlyGenerated && !error) { // No key exists state
     displayValue = ""; // Render nothing inside, placeholder handled by CSS potentially
     currentInputStyle = inputStyleEmpty; // Use empty style (might affect background/text color)
  } else if (isNewlyGenerated) { // New key state
      // displayValue is already set to the new key
      currentInputStyle = inputStyleFilled; // Style for content
  } else if (apiKeyId && !isNewlyGenerated) { // Existing key state (stars)
      displayValue = "*".repeat(64);
      currentInputStyle = inputStyleFilled;
  }
  // Note: Error state doesn't directly change input display here, uses separate message

  return (
    <div>
      <h3 className="text-lg font-medium mb-4">API Key</h3>
      <p className="text-sm text-muted-foreground mb-6">
        Manage your API key for external integrations. Generating a new key will invalidate the current one.
      </p>
      <div className="space-y-4">
        {/* Key Display Area */}
        <div className="relative group w-full"> {/* Ensure parent takes width */}
          <div
            className={`${currentInputStyle} pr-10`} // Apply dynamic style, ensure padding for button
            aria-live="polite" // Announce changes for screen readers
          >
             {/* Render placeholder text via CSS ::before or keep div empty */}
             {displayValue || <span className="text-muted-foreground italic">No API Key generated</span>}
             {/* The above line adds placeholder text if displayValue is empty */}
          </div>

          {/* Copy Button - Show only when a new key is visible */}
          {isNewlyGenerated && !isLoading && (
            <button
              onClick={handleCopy}
              className="absolute top-1/2 right-2 transform -translate-y-1/2 text-gray-400 hover:text-white p-1 rounded hover:bg-gray-700 opacity-50 group-hover:opacity-100 transition-opacity duration-200"
              aria-label={copied ? "API Key Copied" : "Copy API Key"}
              title={copied ? "Copied!" : "Copy API Key"}
              disabled={isLoading}
            >
              {copied ? <Check size={18} className="text-green-500" /> : <Copy size={18} />}
            </button>
          )}
        </div>

        {/* Action Button */}
        <Button onClick={handleGenerateNewKey} disabled={isLoading}>
          {buttonText}
        </Button>

        {/* Informational Messages */}
        {error && (
          <p className="text-xs text-red-500 flex items-center gap-1">
             <AlertCircle size={14} /> Error: {error}
          </p>
        )}
        {copied && (
          <p className="text-xs text-blue-500">Copied to clipboard!</p>
        )}
      </div>
    </div>
  );
}