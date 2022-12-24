import React, { useMemo, useState } from "react";
import { useStores } from "@stores";
import { Column, Row } from "@components/Flex";
import SquareTokenIcon from "@components/SquareTokenIcon";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import BN from "@src/utils/BN";
import Button from "@components/Button";
import { FAUCET_URL } from "@src/constants";
import { useFaucetVM } from "@screens/Faucet/FaucetVm";
import { observer } from "mobx-react-lite";
import Table from "@components/Table";
import Scrollbar from "@components/Scrollbar";
import Loading from "@components/Loading";

interface IProps {}

const TokensFaucetTable: React.FC<IProps> = () => {
  const { accountStore, settingsStore } = useStores();
  const vm = useFaucetVM();
  const [tokens, setTokens] = useState<any>([]);
  useMemo(() => {
    setTokens(
      vm.faucetTokens.map((t) => ({
        asset: (
          <Row>
            <SquareTokenIcon size="small" src={t.logo} alt="logo" />
            <SizedBox width={16} />
            <Column crossAxisSize="max">
              <Text size="medium" fitContent style={{ whiteSpace: "nowrap" }}>
                {t.name}
              </Text>
              <Text
                fitContent
                style={{ whiteSpace: "nowrap" }}
                type="secondary"
                size="small"
              >
                $ {new BN(t.defaultPrice ?? 0).toFormat()}
              </Text>
            </Column>
          </Row>
        ),
        amount: (
          <Column crossAxisSize="max">
            <Text fitContent style={{ whiteSpace: "nowrap" }} weight={500}>
              {`${t.mintAmount.toFormat()} ${t.symbol}`}
            </Text>
            <Text fitContent style={{ whiteSpace: "nowrap" }} type="secondary">
              $ {t.mintAmountDollar.toFormat()}
            </Text>
          </Column>
        ),
        balance: (
          <Column crossAxisSize="max">
            <Text fitContent style={{ whiteSpace: "nowrap" }} weight={500}>
              {`${t.formatBalance?.toFormat(2)} ${t.symbol}`}
            </Text>
            <Text fitContent style={{ whiteSpace: "nowrap" }} type="secondary">
              $ {t.balanceDollar.toFormat(2)}
            </Text>
          </Column>
        ),
        btn: (() => {
          if (accountStore.address == null)
            return (
              <Button
                size="medium"
                fixed
                onClick={() => settingsStore.setLoginModalOpened(true)}
              >
                Connect wallet
              </Button>
            );
          if (vm.alreadyMintedTokens.includes(t.assetId))
            return (
              <Button fixed size="medium" disabled>
                Already minted
              </Button>
            );
          return (
            <Button
              fixed
              size="medium"
              disabled={vm.loading}
              onClick={() => {
                if (t.symbol === "ETH") {
                  window.open(
                    `${FAUCET_URL}/?address=${accountStore.address}`,
                    "blank"
                  );
                } else {
                  vm.mint(t.assetId);
                }
              }}
            >
              {vm.loading ? <Loading /> : "Mint"}
            </Button>
          );
        })(),
      }))
    );
  }, [accountStore.address, settingsStore, vm.loading, vm.alreadyMintedTokens]);
  const columns = React.useMemo(
    () => [
      {
        Header: "Asset",
        accessor: "asset",
      },
      {
        Header: "Mint amount",
        accessor: "amount",
      },
      {
        Header: "My balance",
        accessor: "balance",
      },
      {
        Header: " ",
        accessor: "btn",
      },
    ],
    []
  );
  return (
    <Scrollbar style={{ maxWidth: "calc(100vw - 32px)", borderRadius: 16 }}>
      <Table
        columns={columns}
        data={tokens}
        style={{
          whiteSpace: "nowrap",
          width: "fitContent",
          minWidth: "fit-content",
        }}
      />
    </Scrollbar>
  );
};
export default observer(TokensFaucetTable);