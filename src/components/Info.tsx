import Image from "next/image";
import Link from "next/link";
import React from "react";

import { Button } from "@/components/ui/button";

interface InfoSingleProps {
  title: string;
  text: string;
  isButton: boolean;
}
function InfoSingle({ title, text, isButton }: InfoSingleProps) {
  return (
    <div className="rounded-lg border-2 border-wf-yellow bg-white bg-opacity-80 px-4 py-3 shadow-sm">
      <div className="flex items-center justify-between pb-1">
        <div className="text-body-title text-wf-red">{title}</div>
        {isButton && (
          <Link href="/staking">
            <Button size="S">
              <p className="text-body2_bold">DEPOSIT &gt;</p>
            </Button>
          </Link>
        )}
      </div>
      <div className="flex items-center space-x-1">
        <Image src="/icon_dollar.png" width={32} height={32} alt="Dollar" />
        <h1>{text}</h1>
      </div>
    </div>
  );
}

interface InfoDoubleProps {
  text1a: string;
  text1b: string;
  image1: string;
  alt1: string;
  text2a: string;
  text2b: string;
  image2: string;
  alt2: string;
  image3?: string;
  optionStyle?: string[];
  isSol?: boolean;
}

/**
 * Has been made reusable by accepting props
 */
function InfoDouble({
  text1a,
  text1b,
  image1,
  alt1,
  text2a,
  text2b,
  image2,
  alt2,
  image3 = undefined,
  optionStyle = ["text-center"],
  isSol = false,
}: InfoDoubleProps) {
  return (
    <div className="rounded-lg border-2 border-wf-yellow">
      <div className="bg-white bg-opacity-80 shadow-sm">
        <div className="flex items-center justify-between space-y-[6px] px-4 pb-1 pt-3">
          <div
            className={`text-body-title flex-1 text-center text-wf-red ${optionStyle[0]}`}
          >
            {text1a}
          </div>
        </div>
        <div className={`flex items-end space-x-1 px-4 pb-3 ${optionStyle[1]}`}>
          <div className="flex h-10 items-center">
            <Image src={image1} width={32} height={32} alt={alt1} />
          </div>
          <h1>{text1b}</h1>
          {isSol && (
            <h3 className="flex h-10 items-end text-muted-foreground">SOL</h3>
          )}
        </div>
      </div>
      <div className="mx-4 border-t-2 border-wf-yellow"></div>
      <div className="bg-white bg-opacity-80 shadow-sm">
        <div className="flex items-center justify-between space-y-[6px] px-4 pb-1 pt-3">
          <div
            className={`text-body-title flex-1 text-wf-red ${optionStyle[0]}`}
          >
            {text2a}
          </div>
        </div>
        <div
          className={`flex items-center space-x-1 px-4 pb-3 ${optionStyle[1]}`}
        >
          <Image src={image2} width={24} height={24} alt={alt2} />
          <h2>{text2b}</h2>
          {image3 && <Image src={image3} width={24} height={24} alt={alt2} />}
        </div>
      </div>
    </div>
  );
}

export { InfoSingle, InfoDouble };
