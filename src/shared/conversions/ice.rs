use std::convert::{From, TryFrom};

use mediasoup::types::data_structures::{
    IceCandidateTcpType as MediasoupIceCandidateTcpType,
    IceCandidateType as MediasoupIceCandidateType,
};
use mediasoup::prelude::{
    IceCandidate as MediasoupIceCandidate,
    IceParameters as MediasoupIceParameters,
    Protocol as MediasoupProtocol,
};

use crate::shared::{
    ice_candidate,
    ice_parameters,
    IceCandidate,
    IceCandidateTcpType,
    IceCandidateType,
    IceParameters,
    Protocol,
};
use super::error::ConversionError;

impl TryFrom<Protocol> for MediasoupProtocol {
    type Error = ConversionError;

    fn try_from(value: Protocol) -> Result<Self, Self::Error> {
        match value {
            Protocol::Tcp => Ok(Self::Tcp),
            Protocol::Udp => Ok(Self::Udp),
            Protocol::Unspecified => Err(ConversionError::InvalidIceProtocol),
        }
    }
}

impl From<MediasoupProtocol> for Protocol {
    fn from(value: MediasoupProtocol) -> Self {
        match value {
            MediasoupProtocol::Tcp => Self::Tcp,
            MediasoupProtocol::Udp => Self::Udp,
        }
    }
}

impl TryFrom<IceCandidateType> for MediasoupIceCandidateType {
    type Error = ConversionError;

    fn try_from(value: IceCandidateType) -> Result<Self, Self::Error> {
        match value {
            IceCandidateType::Host => Ok(Self::Host),
            IceCandidateType::Srflx => Ok(Self::Srflx),
            IceCandidateType::Prflx => Ok(Self::Prflx),
            IceCandidateType::Relay => Ok(Self::Relay),
            IceCandidateType::Unspecified => Err(ConversionError::InvalidIceCandidateType),
        }
    }
}

impl From<MediasoupIceCandidateType> for IceCandidateType {
    fn from(value: MediasoupIceCandidateType) -> Self {
        match value {
            MediasoupIceCandidateType::Host => Self::Host,
            MediasoupIceCandidateType::Srflx => Self::Srflx,
            MediasoupIceCandidateType::Prflx => Self::Prflx,
            MediasoupIceCandidateType::Relay => Self::Relay,
        }
    }
}

impl TryFrom<IceCandidateTcpType> for MediasoupIceCandidateTcpType {
    type Error = ConversionError;

    fn try_from(value: IceCandidateTcpType) -> Result<Self, Self::Error> {
        match value {
            IceCandidateTcpType::Passive => Ok(Self::Passive),
            IceCandidateTcpType::Unspecified => Err(ConversionError::InvalidIceCandidateTcpType),
        }
    }
}

impl From<MediasoupIceCandidateTcpType> for IceCandidateTcpType {
    fn from(value: MediasoupIceCandidateTcpType) -> Self {
        match value {
            MediasoupIceCandidateTcpType::Passive => Self::Passive,
        }
    }
}

impl TryFrom<IceCandidate> for MediasoupIceCandidate {
    type Error = ConversionError;

    fn try_from(candidate: IceCandidate) -> Result<Self, Self::Error> {
        let protocol = Protocol::try_from(candidate.protocol)
            .map_err(|_| ConversionError::InvalidIceProtocol)?
            .try_into()
            .map_err(|_| ConversionError::InvalidIceProtocol)?;


        let r#type = IceCandidateType::try_from(candidate.r#type)
            .map_err(|_| ConversionError::InvalidIceCandidateType)?
            .try_into()
            .map_err(|_| ConversionError::InvalidIceCandidateType)?;

        let tcp_type = match candidate.tcp_type_opt {
            Some(ice_candidate::TcpTypeOpt::TcpType(t)) => {
                Some(IceCandidateTcpType::try_from(t)
                    .map_err(|_| ConversionError::InvalidIceCandidateTcpType)?
                    .try_into()
                    .map_err(|_| ConversionError::InvalidIceCandidateTcpType)?
                )
            }
            None => None,
        };

        let port = u16::try_from(candidate.port)
            .map_err(|_| ConversionError::InvalidPort(candidate.port))?;

        Ok(Self {
            protocol,
            port,
            r#type,
            tcp_type,
            foundation: candidate.foundation,
            priority: candidate.priority,
            address: candidate.address,
        })
    }
}

impl TryFrom<MediasoupIceCandidate> for IceCandidate {
    type Error = ConversionError;

    fn try_from(candidate: MediasoupIceCandidate) -> Result<Self, Self::Error> {
        let protocol = Protocol::from(candidate.protocol) as i32;
        let r#type = IceCandidateType::from(candidate.r#type) as i32;
        let tcp_type_opt = candidate.tcp_type.map(|tcp_type| {
            ice_candidate::TcpTypeOpt::TcpType(IceCandidateTcpType::from(tcp_type) as i32)
        });

        Ok(Self {
            tcp_type_opt,
            protocol,
            r#type,
            foundation: candidate.foundation,
            priority: candidate.priority,
            address: candidate.address,
            port: u32::from(candidate.port),
        })
    }
}

impl TryFrom<IceParameters> for MediasoupIceParameters {
    type Error = ConversionError;

    fn try_from(params: IceParameters) -> Result<Self, Self::Error> {
        if params.username_fragment.is_empty() {
            return Err(ConversionError::InvalidUsernameFragment(params.username_fragment));
        }

        if params.password.is_empty() {
            return Err(ConversionError::InvalidPassword(params.password));
        }

        let ice_lite = params.ice_lite_opt.map(|ice_parameters::IceLiteOpt::IceLite(val)| val);

        Ok(Self {
            ice_lite,
            username_fragment: params.username_fragment,
            password: params.password,
        })
    }
}

impl From<MediasoupIceParameters> for IceParameters {
    fn from(params: MediasoupIceParameters) -> Self {
        let ice_lite_opt = params.ice_lite.map(ice_parameters::IceLiteOpt::IceLite);

        Self {
            ice_lite_opt,
            username_fragment: params.username_fragment,
            password: params.password,
        }
    }
}
