import React from "react";
import { AccountContent } from "./AccountContent";

export const Account = () => {
  return (
    <div className="p-5 mx-auto max-w-5xl space-y-5">
      <h1 className="text-4xl">Manage Your Account</h1>
      <AccountContent />
    </div>
  );
};
