// "use client"

// import { useState, useEffect } from "react"
// import { motion } from "framer-motion"
// import { Check, Loader2 } from "lucide-react"
// import { useRouter } from "next/navigation"

// const API_URL = process.env.NEXT_PUBLIC_API_URL;

// export default function VerifyIgnPage() {
//   const [ign, setIgn] = useState("")
//   const [loading, setLoading] = useState(false)
//   const [status, setStatus] = useState<"idle" | "success" | "error">("idle")
//   const [submitted, setSubmitted] = useState(false)
//   const [isComplete, setIsComplete] = useState(false)
//   const router = useRouter()

//   useEffect(() => {
//     // Trigger the completion animation after a short delay
//     const timer = setTimeout(() => setIsComplete(true), 300)
//     return () => clearTimeout(timer)
//   }, [])

//   const handleVerify = async (event: React.FormEvent) => {
//     event.preventDefault()

//     if (!ign.trim()) return

//     setLoading(true)
//     setStatus("idle")
//     setSubmitted(true)

//     try {
//       const response = await fetch(`${API_URL}/api/auth/verify-ign/`, {
//         method: "POST",
//         headers: { "Content-Type": "application/json" },
//         body: JSON.stringify({ ign }),
//         credentials: "include"
//       })

//       if (!response.ok) {
//         throw new Error("IGN verification failed")
//       }

//       const result = await response.json()
//       setStatus("success")

//       setTimeout(() => router.push("/success"), 1500)
//     } catch {
//       setStatus("error")
//     } finally {
//       setLoading(false)
//     }
//   }

//   const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
//     setIgn(e.target.value)
//     setSubmitted(false)
//     setStatus("idle")
//   }

//   const renderButtonContent = () => {
//     if (loading) {
//       return (
//         <>
//           <Loader2 className="mr-2 h-4 w-4 animate-spin" />
//           Verifying...
//         </>
//       )
//     }
//     return status === "error" ? "Invalid IGN" : "Verify"
//   }

//   const renderForm = () => (
//     <form className="w-full max-w-md p-6" onSubmit={handleVerify}>
//       <label htmlFor="ign" className="block text-sm font-medium mb-2 text-foreground">
//         Verify ign
//       </label>
//       <input
//         id="ign"
//         type="text"
//         value={ign}
//         onChange={handleInputChange}
//         className="w-full px-2 py-2 mb-4 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-background text-foreground"
//         placeholder="Enter ign"
//       />
//       <button
//         type="submit"
//         disabled={loading || !ign.trim() || submitted}
//         className={`w-full py-2 rounded-md flex items-center justify-center transition-colors ${
//           status === "error"
//             ? "bg-red-300 text-red-600"
//             : !ign.trim() || submitted
//             ? "bg-gray-800/20 text-gray-500 cursor-not-allowed"
//             : "bg-primary text-primary-foreground hover:bg-primary/90"
//         }`}
//       >
//         {renderButtonContent()}
//       </button>
//     </form>
//   )

//   const renderSuccess = () => (
//     <div className="flex flex-col items-center justify-center p-6 gap-4">
//       <div className="relative w-24 h-24">
//         <motion.div
//           className="absolute inset-0 rounded-full bg-green-500 origin-bottom"
//           initial={{ scale: 0, opacity: 0 }}
//           animate={{ scale: isComplete ? 1 : 0, opacity: 1 }}
//           transition={{ duration: 0.7, ease: "circOut" }}
//           style={{ clipPath: "circle(50% at center)" }}
//         />
//         <motion.div
//           className="absolute inset-0 flex items-center justify-center text-white"
//           initial={{ scale: 0, opacity: 0 }}
//           animate={{ scale: isComplete ? 1 : 0, opacity: isComplete ? 1 : 0 }}
//           transition={{ delay: 0.5, duration: 0.3, type: "spring", stiffness: 200 }}
//         >
//           <Check className="w-12 h-12 stroke-[3]" />
//         </motion.div>
//       </div>
//       <motion.div
//         className="text-center"
//         initial={{ opacity: 0, y: 10 }}
//         animate={{ opacity: 1, y: 0 }}
//         transition={{ delay: 0.8, duration: 0.3 }}
//       >
//         <h3 className="text-xl font-medium text-green-500">IGN Verified</h3>
//       </motion.div>
//     </div>
//   )

//   return (
//     <main className="flex min-h-screen flex-col items-center justify-center p-4 bg-background">
//       {status === "success" ? renderSuccess() : renderForm()}
//     </main>
//   )
// }
