"use client";

import { useWallet } from "@solana/wallet-adapter-react";
import React, { useEffect, useState } from "react";

import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/hooks/useAuth";
import { RankingUserProps, usePoints } from "@/hooks/usePoints";

import Loading from "../../../components/Loading";
import PopupResultDeposit from "../../../components/PopupResultDeposit";
import { useVault } from "../hook/useVault";

export type DialogState =
  | "none"
  | "confirm"
  | "loading"
  | "depositResult"
  | "withdrawResult"
  | "error";

interface AlertProp {
  actionType: "deposit" | "withdraw";
  amount: string;
}

export function Popup({ actionType, amount }: AlertProp) {
  const [dialogState, setDialogState] = useState<DialogState>("none");
  // const [isLoading, setIsLoading] = useState(false);
  const { publicKey, sendTransaction } = useWallet();

  const context = useAuth();
  const { addPoints, user, setUser } = usePoints();
  const { handleDeposit, handleWithdraw } = useVault({
    amount,
    setDialogState,
  });

  const userId = context.userId.slice(0, 4) + ".." + context.userId.slice(-4);
  useEffect(() => {
    const fetchRankings = async () => {
      try {
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_FAST_API_URL}/api/rankings`,
          {
            method: "GET",
          },
        );
        const data = await response.json();
        const user = data.rankings.find(
          (user: RankingUserProps) => user.name === userId,
        );
        setUser(user);
      } catch (error) {
        console.error("Error fetching rankings:", error);
      }
    };
    fetchRankings();
  }, [userId, setUser]);

  const handleButtonClick = () => {
    if (!amount || parseFloat(amount) <= 0) {
      setDialogState("error");
      return;
    }
    setDialogState("confirm");
  };

  const handleTransaction = async () => {
    setDialogState("loading");

    try {
      await runTransaction();
      setDialogState(
        actionType === "deposit" ? "depositResult" : "withdrawResult",
      );
    } catch (error) {
      console.error(error);
      setDialogState("error");
    }
  };

  const runTransaction = async (): Promise<string | undefined> => {
    if (!publicKey || !sendTransaction) {
      throw new Error("Wallet is not connected.");
    }
    if (actionType === "deposit") {
      const signature = await handleDeposit();
      if (!signature) {
        throw new Error("Deposit transaction failed.");
      }

      await addPoints(signature, context.userId);
      return signature;
    }
    if (actionType === "withdraw") {
      console.log("withdraw is being processed.");
      const signature = await handleWithdraw();
      if (!signature) {
        throw new Error("Withdraw transaction failed.");
      }
      return signature;
    }
  };

  const resetDialog = () => {
    setDialogState("none");
  };

  return (
    <div className="relative">
      {/* Button to start the process */}
      <Button variant="standard" className="w-full" onClick={handleButtonClick}>
        {actionType}
      </Button>

      {/* Confirm Dialog */}
      {dialogState === "confirm" && (
        <AlertDialog open onOpenChange={() => setDialogState("none")}>
          <AlertDialogContent className="mx-auto w-[335px] rounded-2xl p-6">
            <AlertDialogHeader>
              <AlertDialogTitle className="text-left">
                Are you sure?
              </AlertDialogTitle>
              <AlertDialogDescription className="body text-left">
                {`You are about to ${actionType} ${amount} SOL.`}
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter className="flex flex-row items-center justify-end space-x-[10px]">
              <AlertDialogCancel onClick={() => setDialogState("none")}>
                Cancel
              </AlertDialogCancel>
              <Button onClick={handleTransaction}>Continue</Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}

      {/* Loading Dialog */}
      {dialogState === "loading" && <Loading />}

      {/* DepositResult Dialog */}
      {dialogState === "depositResult" && (
        <AlertDialog open onOpenChange={() => setDialogState("none")}>
          <AlertDialogContent
            style={{
              margin: "unset",
              padding: "unset",
              border: "unset",
              borderRadius: "12px",
            }}
          >
            <PopupResultDeposit resetDialog={resetDialog} user={user} />
          </AlertDialogContent>
        </AlertDialog>
      )}

      {/* WithdrawResult Dialog */}
      {dialogState === "withdrawResult" && (
        <AlertDialog open onOpenChange={() => setDialogState("none")}>
          <AlertDialogContent className="rounded-2xl">
            <AlertDialogHeader>
              <AlertDialogTitle className="text-center">
                Withdrawal has been completed.
              </AlertDialogTitle>
            </AlertDialogHeader>
            <AlertDialogFooter className="flex w-full flex-row items-center">
              <Button
                size="M"
                onClick={() => setDialogState("none")}
                className="w-full"
              >
                OK
              </Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}

      {/* Error Dialog */}
      {dialogState === "error" && (
        <AlertDialog open onOpenChange={() => setDialogState("none")}>
          <AlertDialogContent className="mx-auto w-[335px] rounded-2xl p-6">
            <AlertDialogHeader>
              <AlertDialogTitle className="text-left">
                Transaction Failed
              </AlertDialogTitle>
              <AlertDialogDescription className="body text-left">
                Please ensure all fields are valid and your wallet is connected.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter className="flex flex-row items-center justify-end space-x-[10px]">
              <Button onClick={() => setDialogState("none")}>Close</Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}
    </div>
  );
}
