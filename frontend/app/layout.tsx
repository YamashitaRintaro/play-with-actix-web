import type { Metadata } from "next";
import { Bitter } from "next/font/google";
import "./globals.css";

const bitter = Bitter({
  variable: "--font-bitter",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Twitter Clone",
  description: "A Twitter clone built with Next.js and Actix Web",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={`${bitter.variable} antialiased`}>{children}</body>
    </html>
  );
}
