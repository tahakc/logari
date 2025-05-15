import { redirect } from "next/navigation"
import { createClient } from "@/lib/supabase/server"
import { AuthForm } from "@/components/auth/auth-form"
import { AuthCard } from "@/components/auth/auth-card"

export default async function LoginPage() {
  const supabase = await createClient()
  const { data } = await supabase.auth.getSession()

  if (data.session) {
    redirect("/dashboard")
  }

  return (
    <AuthCard>
      <AuthForm mode="login" />
    </AuthCard>
  )
}
