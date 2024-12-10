import Image from "next/image";
import Link from "next/link";
import React from "react";

import { RankingUserProps } from "@/hooks/usePoints";

import { InfoDouble } from "./Info";

interface PopupResultDepositProps {
  resetDialog: () => void;
  user: RankingUserProps;
}

function PopupResultDeposit({ resetDialog, user }: PopupResultDepositProps) {
  return (
    <div>
      <div className="flex w-[335px] flex-col justify-between space-y-6 px-[30px] py-10">
        <h2 className="text-center">Deposit has been completed.</h2>
        <div className="space-y-2">
          <p className="text-body-bold text-center text-wf-orange">
            Your Results
          </p>
          <InfoDouble
            text1a="Earned Points"
            text1b={user.points}
            image1="/icon_point.png"
            alt1="Dollar"
            text2a="Ranking"
            text2b={user.rank}
            image2="/icon_ranking.png"
            alt2="Dollar"
            image3="/icon_up.png"
          />
        </div>
        <Link href="/">
          <Image
            src="/banner_sample.png"
            alt="banner_sample"
            width={271}
            height={80}
          />
        </Link>
      </div>
      <Image
        src="/Button_close.png"
        alt="button_close"
        height={40}
        width={40}
        className="absolute right-2 top-4"
        onClick={resetDialog}
      />
    </div>
  );
}

export default PopupResultDeposit;
